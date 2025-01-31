use steel::*;

use super::ProtobookAccount;

/// Order tracks an offer to exchange two tokens.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Order {
    pub authority: Pubkey,

    pub amount_a: u64,

    pub amount_b: u64,

    pub commission: u64,

    pub commission_recipient: Pubkey,
    
    pub expires_at: i64,

    pub filled: u64,

    pub mint_a: Pubkey,

    pub mint_b: Pubkey,

    pub seed: [u8; 32],

    pub threshold: u64,

    pub total_receipts: u64,
}

account!(ProtobookAccount, Order);