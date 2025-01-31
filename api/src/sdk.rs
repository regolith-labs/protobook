use steel::*;

use crate::prelude::*;

pub fn open(authority: Pubkey, amount_a: u64, amount_b: u64, commission: u64, expires_at: i64, mint_a: Pubkey, mint_b: Pubkey, seed: [u8; 32], threshold: u64) -> Instruction {
    let sender = spl_associated_token_account::get_associated_token_address(&authority, &mint_a);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(order_pda(authority, seed).0, false),
            AccountMeta::new(sender, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Open {
            amount_a,
            amount_b,
            commission,
            expires_at,
            seed,
            threshold,
        }.to_bytes()
    }
}
