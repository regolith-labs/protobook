use steel::*;

use super::ProtobookAccount;

/// A receipt is a deposit to fill an order.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Receipt { 
    /// The authority of the receipt. 
    pub authority: Pubkey,

    /// The amount of token B deposited by the receipt holder.
    pub deposit: u64,

    /// The order that the receipt is redeemable for.
    pub order: Pubkey,
}

account!(ProtobookAccount, Receipt);
