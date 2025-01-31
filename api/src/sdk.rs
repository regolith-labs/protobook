use steel::*;

use crate::prelude::*;

pub fn open(authority: Pubkey, amount_a: u64, amount_b: u64, expires_at: i64, fee: u64, id: [u8; 32], mint_a: Pubkey, mint_b: Pubkey) -> Instruction {
    let sender = spl_associated_token_account::get_associated_token_address(&authority, &mint_a);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(mint_a, false),
            AccountMeta::new(mint_b, false),
            AccountMeta::new(order_pda(authority, id).0, false),
            AccountMeta::new(sender, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Open {
            amount_a,
            amount_b,
            expires_at,
            fee,
            id,
        }.to_bytes()
    }
}
