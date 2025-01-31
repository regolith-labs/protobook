use protobook_api::prelude::*;
use steel::*;

/// Closes an order.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, order_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.authority == *signer_info.key)?
        .assert_mut(|o| o.expires_at > clock.unix_timestamp)?;
    system_program.is_program(&system_program::ID)?;

    // Marks the order as immediately expired.
    order.expires_at = clock.unix_timestamp;

    Ok(())
}
