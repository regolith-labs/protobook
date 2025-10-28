mod order;
mod receipt;
pub use order::*;
pub use receipt::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum ProtobookAccount {
    Order = 0,
    Receipt = 1,
}

/// Fetch PDA of the order account.
pub fn order_pda(authority: Pubkey, id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[ORDER, authority.as_ref(), &id.to_le_bytes()],
        &crate::id(),
    )
}

/// Fetch PDA of the receipt account.
pub fn receipt_pda(authority: Pubkey, order: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[RECEIPT, authority.as_ref(), order.as_ref()], &crate::id())
}
