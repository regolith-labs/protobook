use steel::*;

use super::ProtobookAccount;

/// An order is a public, time-bound offer to exchange two tokens at a fixed price.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Order {
    /// The creator of the order.
    pub authority: Pubkey,

    /// The amount of token A offered by the authority and locked in escrow.
    pub amount_a: u64,

    /// The amount of token B requested by the authority.
    pub amount_b: u64,

    /// The time at which the order expires.
    pub expires_at: i64,

    /// An optional fee to be paid by the authority if the order is filled.
    pub fee: u64,

    /// The collector of the fee.
    pub fee_collector: Pubkey,

    /// The unique identifier of the order.
    pub id: [u8; 32],

    /// The mint of token A.
    pub mint_a: Pubkey,

    /// The mint of token B.
    pub mint_b: Pubkey,

    /// The total amount of token B deposited by the public.
    pub total_deposits: u64,
}

account!(ProtobookAccount, Order);