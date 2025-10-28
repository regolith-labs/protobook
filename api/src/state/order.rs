use steel::*;

use super::ProtobookAccount;

/// An order is a public, timebound offer to buy a given token at a fixed price.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Order {
    /// The maker of the order.
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

    /// A unique identifier for the order, namespaced by the authority.
    pub id: u64,

    /// The mint of token A.
    pub mint_a: Pubkey,

    /// The mint of token B.
    pub mint_b: Pubkey,

    /// The total amount of token B deposited by takers.
    pub total_deposits: u64,

    /// The total number receipts issued for this order.
    pub total_receipts: u64,

    /// The total amount of receipts redeemed by takers.
    pub total_redeemed: u64,

    /// Is collected.
    pub is_collected: u64,
}

account!(ProtobookAccount, Order);
