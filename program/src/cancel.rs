use protobook_api::prelude::*;
use steel::*;

/// Cancels an order immediately.
pub fn process_cancel(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, order_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.authority == *signer_info.key)?
        .assert_mut(|o| o.expires_at > clock.unix_timestamp)?;

    // Marks the order as immediately expired.
    order.expires_at = clock.unix_timestamp;

    Ok(())
}
