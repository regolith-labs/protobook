use spl_associated_token_account::get_associated_token_address;
use steel::*;

use crate::prelude::*;

// let [signer_info, order_info] = accounts else {

pub fn cancel(authority: Pubkey, id: [u8; 32]) -> Instruction {
    let order_address = order_pda(authority, id).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(order_address, false),
        ],
        data: Cancel {}.to_bytes(),
    }
}

// let [signer_info, beneficiary_a_info, beneficiary_b_info, mint_a_info, mint_b_info, order_info, vault_a_info, vault_b_info, system_program, token_program, associated_token_program] =

pub fn close(authority: Pubkey, id: [u8; 32], mint_a: Pubkey, mint_b: Pubkey) -> Instruction {
    let order_address = order_pda(authority, id).0;
    let beneficiary_a = get_associated_token_address(&authority, &mint_a);
    let beneficiary_b = get_associated_token_address(&authority, &mint_b);
    let vault_a = get_associated_token_address(&order_address, &mint_a);
    let vault_b = get_associated_token_address(&order_address, &mint_b);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(beneficiary_a, false),
            AccountMeta::new(beneficiary_b, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(order_address, false),
            AccountMeta::new(vault_a, false),
            AccountMeta::new(vault_b, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Close {}.to_bytes(),
    }
}

// let [signer_info, beneficiary_info, fee_collector_info, mint_info, order_info, vault_info, system_program, token_program, associated_token_program] =

pub fn collect(
    authority: Pubkey,
    beneficiary: Pubkey,
    fee_collector: Pubkey,
    id: [u8; 32],
    mint: Pubkey,
) -> Instruction {
    let order_address = order_pda(authority, id).0;
    let vault = get_associated_token_address(&order_address, &mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(fee_collector, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(order_address, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Collect {}.to_bytes(),
    }
}

// let [signer_info, order_info, receipt_info, sender_info, vault_b_info, system_program, token_program] =

pub fn fill(authority: Pubkey, id: [u8; 32], mint_b: Pubkey, amount: u64) -> Instruction {
    let order_address = order_pda(authority, id).0;
    let vault_b = get_associated_token_address(&order_address, &mint_b);
    let receipt_address = receipt_pda(authority, order_address).0;
    let sender = get_associated_token_address(&authority, &mint_b);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(order_address, false),
            AccountMeta::new(receipt_address, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(vault_b, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Fill { amount }.to_bytes(),
    }
}

// let [signer_info, fee_collector_info, mint_a_info, mint_b_info, order_info, sender_info, vault_a_info, vault_b_info, system_program, token_program, associated_token_program] =

pub fn open(
    authority: Pubkey,
    fee_collector: Pubkey,
    amount_a: u64,
    amount_b: u64,
    expires_at: i64,
    fee: u64,
    id: [u8; 32],
    mint_a: Pubkey,
    mint_b: Pubkey,
) -> Instruction {
    let sender = get_associated_token_address(&authority, &mint_a);
    let order_address = order_pda(authority, id).0;
    let vault_a = get_associated_token_address(&order_address, &mint_a);
    let vault_b = get_associated_token_address(&order_address, &mint_b);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(fee_collector, false),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(order_address, false),
            AccountMeta::new(sender, false),
            AccountMeta::new(vault_a, false),
            AccountMeta::new(vault_b, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Open {
            amount_a,
            amount_b,
            expires_at,
            fee,
            id,
        }
        .to_bytes(),
    }
}

// let [signer_info, beneficiary_info, mint_info, order_info, receipt_info, vault_info, system_program, token_program, associated_token_program] =

pub fn redeem(authority: Pubkey, beneficiary: Pubkey, id: [u8; 32], mint: Pubkey) -> Instruction {
    let order_address = order_pda(authority, id).0;
    let receipt_address = receipt_pda(authority, order_address).0;
    let vault = get_associated_token_address(&order_address, &mint);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(order_address, false),
            AccountMeta::new(receipt_address, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Redeem {}.to_bytes(),
    }
}
