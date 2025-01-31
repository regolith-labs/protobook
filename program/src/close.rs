use protobook_api::prelude::*;
use steel::*;

/// Closes an order.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, order_info, vault_a_info, vault_b_info, system_program, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.authority == *signer_info.key)?
        .assert_mut(|o| o.expires_at < clock.unix_timestamp)?;
    vault_a_info
        .is_writable()?
        .as_associated_token_account(&order_info.key, &order.mint_a)?
        .assert(|v| v.amount == 0)?;
    vault_b_info
        .is_writable()?
        .as_associated_token_account(&order_info.key, &order.mint_b)?
        .assert(|v| v.amount == 0)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
 
    // Close order account.
    order_info.close(signer_info)?;

    // Close vault accounts.
    close_token_account_signed(
        &vault_a_info,
        &signer_info,
        &order_info,
        &token_program,
        &[&ORDER, signer_info.key.as_ref(), order.id.as_ref()],
    )?;
    close_token_account_signed(
        &vault_b_info,
        &signer_info,
        &order_info,
        &token_program,
        &[&ORDER, signer_info.key.as_ref(), order.id.as_ref()],
    )?;

    Ok(())
}
