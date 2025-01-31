use steel::*;

use super::ProtobookAccount;

/// Receipt tracks an offer to fill an order.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Receipt { 
    pub authority: Pubkey,

    pub balance: u64,

    pub order: Pubkey,
}

account!(ProtobookAccount, Receipt);
