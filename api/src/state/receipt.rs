use steel::*;

use super::ProtobookAccount;

/// A receipt tracks a deposit to fill an order.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Receipt {
    /// The holder of the receipt (order taker).
    pub authority: Pubkey,

    /// The amount of token B deposited by the taker.
    pub deposit: u64,

    /// The order this receipt is redeemable for.
    pub order: Pubkey,
}

account!(ProtobookAccount, Receipt);
