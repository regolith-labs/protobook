use std::str::FromStr;

use protobook_api::prelude::*;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    client_error::{reqwest::StatusCode, ClientErrorKind},
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
};
use solana_sdk::{
    address_lookup_table::AddressLookupTableAccount,
    compute_budget::ComputeBudgetInstruction,
    message::{v0::Message, VersionedMessage},
    pubkey::Pubkey,
    signature::{read_keypair_file, Signature, Signer},
    transaction::{Transaction, VersionedTransaction},
};
use spl_associated_token_account::get_associated_token_address;
use steel::{AccountDeserialize, Clock, Discriminator, Instruction};

#[tokio::main]
async fn main() {
    // Read keypair from file
    let payer =
        read_keypair_file(&std::env::var("KEYPAIR").expect("Missing KEYPAIR env var")).unwrap();

    // Build transaction
    let rpc = RpcClient::new(std::env::var("RPC").expect("Missing RPC env var"));
    match std::env::var("COMMAND")
        .expect("Missing COMMAND env var")
        .as_str()
    {
        "clock" => {
            log_clock(&rpc).await.unwrap();
        }
        "open" => {
            open(&rpc, &payer).await.unwrap();
        }
        "fill" => {
            fill(&rpc, &payer).await.unwrap();
        }
        "receipt" => {
            log_receipt(&rpc).await.unwrap();
        }
        "cancel" => {
            cancel(&rpc, &payer).await.unwrap();
        }
        "collect" => {
            collect(&rpc, &payer).await.unwrap();
        }
        "redeem" => {
            redeem(&rpc, &payer).await.unwrap();
        }
        "close" => {
            close(&rpc, &payer).await.unwrap();
        }
        "order" => {
            log_order(&rpc).await.unwrap();
        }
        _ => panic!("Invalid command"),
    };
}

async fn open(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").unwrap();
    let id = u64::from_str(&id).expect("Invalid ID");
    let amount_a = std::env::var("AMOUNT_A").unwrap();
    let amount_a = u64::from_str(&amount_a).expect("Invalid AMOUNT_A");
    let amount_b = std::env::var("AMOUNT_B").unwrap();
    let amount_b = u64::from_str(&amount_b).expect("Invalid AMOUNT_B");
    let mint_a = std::env::var("MINT_A").unwrap();
    let mint_a = Pubkey::from_str(&mint_a).expect("Invalid MINT_A");
    let mint_b = std::env::var("MINT_B").unwrap();
    let mint_b = Pubkey::from_str(&mint_b).expect("Invalid MINT_B");
    let clock = get_clock(rpc).await?;
    let expires_at = clock.unix_timestamp + (2 * 60 * 60); // 2 hours
    let ix = protobook_api::sdk::open(
        payer.pubkey(),
        amount_a,
        amount_b,
        expires_at,
        0,
        Pubkey::default(),
        id,
        mint_a,
        mint_b,
    );
    submit_transaction(rpc, payer, &[ix]).await?;
    Ok(())
}

async fn fill(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let order_address = std::env::var("ORDER_ADDRESS").unwrap();
    let order_address = Pubkey::from_str(&order_address).expect("Invalid ORDER_ADDRESS");
    let amount = std::env::var("AMOUNT").unwrap();
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");
    let order = get_order(&rpc, order_address).await?;
    let clock = get_clock(rpc).await?;
    if order.expires_at < clock.unix_timestamp {
        return Err(anyhow::anyhow!("Order expired"));
    }
    if order.amount_b <= order.total_deposits {
        return Err(anyhow::anyhow!("Order is filled"));
    }
    let ix = protobook_api::sdk::fill(payer.pubkey(), order_address, order.mint_b, amount);
    submit_transaction(rpc, payer, &[ix]).await?;

    // Log receipt
    let receipt_address = receipt_pda(payer.pubkey(), order_address).0;
    let receipt = get_receipt(&rpc, receipt_address).await?;
    print_receipt(receipt).await?;
    Ok(())
}

async fn cancel(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").unwrap();
    let id = u64::from_str(&id).expect("Invalid ID");
    let ix = protobook_api::sdk::cancel(payer.pubkey(), id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Order cancelled");
    Ok(())
}

// authority: Pubkey,
// beneficiary: Pubkey,
// fee_collector: Pubkey,
// id: u64,
// mint: Pubkey,

async fn collect(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").unwrap();
    let id = u64::from_str(&id).expect("Invalid ID");
    let clock = get_clock(rpc).await?;
    let order_address = order_pda(payer.pubkey(), id).0;
    let order = get_order(&rpc, order_address).await?;
    if order.is_collected == 1 {
        return Err(anyhow::anyhow!("Order is collected"));
    }
    if order.expires_at > clock.unix_timestamp {
        return Err(anyhow::anyhow!("Order is open"));
    }
    let mint = if order.total_deposits == order.amount_b {
        order.mint_b
    } else {
        order.mint_a
    };
    let beneficiary = get_associated_token_address(&order_address, &mint);
    let ix =
        protobook_api::sdk::collect(payer.pubkey(), beneficiary, order.fee_collector, id, mint);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Order collected");
    Ok(())
}

// authority: Pubkey, beneficiary: Pubkey, id: u64, mint: Pubkey

async fn redeem(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").unwrap();
    let id = u64::from_str(&id).expect("Invalid ID");
    let clock = get_clock(rpc).await?;
    let order_address = order_pda(payer.pubkey(), id).0;
    let order = get_order(&rpc, order_address).await?;
    if order.total_redeemed == order.total_receipts {
        return Err(anyhow::anyhow!("Order is redeemed"));
    }
    if order.expires_at > clock.unix_timestamp {
        return Err(anyhow::anyhow!("Order is open"));
    }
    let mint = if order.total_deposits == order.amount_b {
        order.mint_a
    } else {
        order.mint_b
    };
    let beneficiary = get_associated_token_address(&order_address, &mint);
    let ix = protobook_api::sdk::redeem(payer.pubkey(), beneficiary, id, mint);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Receipt redeemed");
    Ok(())
}

// authority: Pubkey, id: u64, mint_a: Pubkey, mint_b: Pubkey
async fn close(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").unwrap();
    let id = u64::from_str(&id).expect("Invalid ID");
    let clock = get_clock(rpc).await?;
    let order_address = order_pda(payer.pubkey(), id).0;
    let order = get_order(&rpc, order_address).await?;
    if order.expires_at > clock.unix_timestamp {
        return Err(anyhow::anyhow!("Order is open"));
    }
    if order.total_receipts != order.total_redeemed {
        return Err(anyhow::anyhow!("Order is not redeemed"));
    }
    if order.is_collected == 0 {
        return Err(anyhow::anyhow!("Order is not collected"));
    }
    let ix = protobook_api::sdk::close(payer.pubkey(), id, order.mint_a, order.mint_b);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Order closed");
    Ok(())
}

async fn log_clock(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let clock = get_clock(&rpc).await?;
    println!("Clock");
    println!("  slot: {}", clock.slot);
    println!("  epoch_start_timestamp: {}", clock.epoch_start_timestamp);
    println!("  epoch: {}", clock.epoch);
    println!("  leader_schedule_epoch: {}", clock.leader_schedule_epoch);
    println!("  unix_timestamp: {}", clock.unix_timestamp);
    Ok(())
}

async fn log_receipt(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let address = std::env::var("ADDRESS").unwrap();
    let address = Pubkey::from_str(&address).expect("Invalid ADDRESS");
    let receipt = get_receipt(&rpc, address).await?;
    print_receipt(receipt).await?;
    Ok(())
}

async fn log_order(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let address = std::env::var("ADDRESS").unwrap();
    let address = Pubkey::from_str(&address).expect("Invalid ADDRESS");
    let order = get_order(&rpc, address).await?;
    let clock = get_clock(&rpc).await?;
    print_order(order, clock);
    Ok(())
}

fn print_order(order: Order, clock: Clock) {
    println!("Order");
    println!("  Id: {:?}", order.id);
    println!("  Amount A: {}", order.amount_a);
    println!("  Amount B: {}", order.amount_b);
    println!("  Expires at: {}", order.expires_at);
    println!("  Fee: {}", order.fee);
    println!("  Fee collector: {}", order.fee_collector);
    println!("  Mint A: {}", order.mint_a);
    println!("  Mint B: {}", order.mint_b);
    println!("  Total deposits: {}", order.total_deposits);
    println!("  Total receipts: {}", order.total_receipts);
    println!("  Total redeemed: {}", order.total_redeemed);
    println!("  Is collected: {}", order.is_collected);
}

async fn print_receipt(receipt: Receipt) -> Result<(), anyhow::Error> {
    println!("Receipt");
    println!("  Authority: {}", receipt.authority);
    println!("  Deposit: {}", receipt.deposit);
    println!("  Order: {}", receipt.order);
    Ok(())
}

async fn get_clock(rpc: &RpcClient) -> Result<Clock, anyhow::Error> {
    let data = rpc.get_account_data(&solana_sdk::sysvar::clock::ID).await?;
    let clock = bincode::deserialize::<Clock>(&data)?;
    Ok(clock)
}

async fn get_order(rpc: &RpcClient, address: Pubkey) -> Result<Order, anyhow::Error> {
    let account = rpc.get_account(&address).await?;
    let order = Order::try_from_bytes(&account.data)?;
    Ok(*order)
}

async fn get_receipt(rpc: &RpcClient, address: Pubkey) -> Result<Receipt, anyhow::Error> {
    let account = rpc.get_account(&address).await?;
    let receipt = Receipt::try_from_bytes(&account.data)?;
    Ok(*receipt)
}

async fn get_receipts(
    rpc: &RpcClient,
    order: Pubkey,
) -> Result<Vec<(Pubkey, Receipt)>, anyhow::Error> {
    //  TODO Filter by order
    let receipts = get_program_accounts::<Receipt>(rpc, protobook_api::ID, vec![]).await?;
    Ok(receipts)
}

async fn get_orders(rpc: &RpcClient) -> Result<Vec<(Pubkey, Order)>, anyhow::Error> {
    let orders = get_program_accounts::<Order>(rpc, protobook_api::ID, vec![]).await?;
    Ok(orders)
}

#[allow(dead_code)]
async fn simulate_transaction(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) {
    let blockhash = rpc.get_latest_blockhash().await.unwrap();
    let x = rpc
        .simulate_transaction(&Transaction::new_signed_with_payer(
            instructions,
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ))
        .await;
    println!("Simulation result: {:?}", x);
}

#[allow(dead_code)]
async fn simulate_transaction_with_address_lookup_tables(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
    address_lookup_table_accounts: Vec<AddressLookupTableAccount>,
) {
    let blockhash = rpc.get_latest_blockhash().await.unwrap();
    let tx = VersionedTransaction {
        signatures: vec![Signature::default()],
        message: VersionedMessage::V0(
            Message::try_compile(
                &payer.pubkey(),
                instructions,
                &address_lookup_table_accounts,
                blockhash,
            )
            .unwrap(),
        ),
    };
    let s = tx.sanitize();
    println!("Sanitize result: {:?}", s);
    s.unwrap();
    let x = rpc.simulate_transaction(&tx).await;
    println!("Simulation result: {:?}", x);
}

#[allow(unused)]
async fn submit_transaction_batches(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    mut ixs: Vec<solana_sdk::instruction::Instruction>,
    batch_size: usize,
) -> Result<(), anyhow::Error> {
    // Batch and submit the instructions.
    while !ixs.is_empty() {
        let batch = ixs
            .drain(..std::cmp::min(batch_size, ixs.len()))
            .collect::<Vec<Instruction>>();
        submit_transaction_no_confirm(rpc, payer, &batch).await?;
    }
    Ok(())
}

#[allow(unused)]
async fn simulate_transaction_batches(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    mut ixs: Vec<solana_sdk::instruction::Instruction>,
    batch_size: usize,
) -> Result<(), anyhow::Error> {
    // Batch and submit the instructions.
    while !ixs.is_empty() {
        let batch = ixs
            .drain(..std::cmp::min(batch_size, ixs.len()))
            .collect::<Vec<Instruction>>();
        simulate_transaction(rpc, payer, &batch).await;
    }
    Ok(())
}

async fn submit_transaction(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;
    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        ComputeBudgetInstruction::set_compute_unit_price(1_000_000),
    ];
    all_instructions.extend_from_slice(instructions);
    let transaction = Transaction::new_signed_with_payer(
        &all_instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction submitted: {:?}", signature);
            Ok(signature)
        }
        Err(e) => {
            println!("Error submitting transaction: {:?}", e);
            Err(e.into())
        }
    }
}

async fn submit_transaction_no_confirm(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;
    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        ComputeBudgetInstruction::set_compute_unit_price(1_000_000),
    ];
    all_instructions.extend_from_slice(instructions);
    let transaction = Transaction::new_signed_with_payer(
        &all_instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    match rpc.send_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction submitted: {:?}", signature);
            Ok(signature)
        }
        Err(e) => {
            println!("Error submitting transaction: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_program_accounts<T>(
    client: &RpcClient,
    program_id: Pubkey,
    filters: Vec<RpcFilterType>,
) -> Result<Vec<(Pubkey, T)>, anyhow::Error>
where
    T: AccountDeserialize + Discriminator + Clone,
{
    let mut all_filters = vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
        0,
        &T::discriminator().to_le_bytes(),
    ))];
    all_filters.extend(filters);
    let result = client
        .get_program_accounts_with_config(
            &program_id,
            RpcProgramAccountsConfig {
                filters: Some(all_filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await;

    match result {
        Ok(accounts) => {
            let accounts = accounts
                .into_iter()
                .filter_map(|(pubkey, account)| {
                    if let Ok(account) = T::try_from_bytes(&account.data) {
                        Some((pubkey, account.clone()))
                    } else {
                        None
                    }
                })
                .collect();
            Ok(accounts)
        }
        Err(err) => match err.kind {
            ClientErrorKind::Reqwest(err) => {
                if let Some(status_code) = err.status() {
                    if status_code == StatusCode::GONE {
                        panic!(
                                "\n{} Your RPC provider does not support the getProgramAccounts endpoint, needed to execute this command. Please use a different RPC provider.\n",
                                "ERROR"
                            );
                    }
                }
                return Err(anyhow::anyhow!("Failed to get program accounts: {}", err));
            }
            _ => return Err(anyhow::anyhow!("Failed to get program accounts: {}", err)),
        },
    }
}
