use protobook_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "protobook_program",
        protobook_api::ID,
        processor!(protobook_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;
    
    // Setup test
    // TODO Initialize mints
    // TODO Mint tokens to sender
    let mint_a = Keypair::new();
    let mint_b = Keypair::new();

    // Submit initialize transaction.
    let ix = open(payer.pubkey(), 100, 100, 10, 0, mint_a.pubkey(), mint_b.pubkey(), [0; 32], 1);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was initialized.
    let order_address = order_pda(payer.pubkey(), [0; 32]).0;
    let order_account = banks.get_account(order_address).await.unwrap().unwrap();
    let order = Order::try_from_bytes(&order_account.data).unwrap();
    assert_eq!(order_account.owner, protobook_api::ID);
    assert_eq!(order.authority, payer.pubkey());
    assert_eq!(order.amount_a, 100);
    assert_eq!(order.amount_b, 100);
    assert_eq!(order.commission, 10);
    assert_eq!(order.commission_recipient, Pubkey::default());
    assert_eq!(order.expires_at, 0);
    assert_eq!(order.filled, 0);
    assert_eq!(order.mint_a, mint_a.pubkey());
    assert_eq!(order.mint_b, mint_b.pubkey());
    assert_eq!(order.seed, [0; 32]);
    assert_eq!(order.threshold, 1);
    assert_eq!(order.total_receipts, 0);
}

