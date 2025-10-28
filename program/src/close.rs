use protobook_api::prelude::*;
use steel::*;

/// Closes an order.
pub fn process_close(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, beneficiary_a_info, beneficiary_b_info, mint_a_info, mint_b_info, order_info, vault_a_info, vault_b_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.authority == *signer_info.key)?
        .assert_mut(|o| o.mint_a == *mint_a_info.key)?
        .assert_mut(|o| o.mint_b == *mint_b_info.key)?
        .assert_mut(|o| o.expires_at < clock.unix_timestamp)?
        .assert_mut(|o| o.total_receipts == o.total_redeemed)?
        .assert_mut(|o| o.is_collected == 1)?;
    let vault_a = vault_a_info
        .is_writable()?
        .as_associated_token_account(&order_info.key, &order.mint_a)?;
    let vault_b = vault_b_info
        .is_writable()?
        .as_associated_token_account(&order_info.key, &order.mint_b)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Transfer any remaining dust to the creator.
    // Dust can only happen due to rounding amounts when redeeming receipts.
    if vault_a.amount() > 0 {
        if beneficiary_a_info.data_is_empty() {
            create_associated_token_account(
                signer_info,
                signer_info,
                beneficiary_a_info,
                mint_a_info,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            beneficiary_a_info.as_associated_token_account(&signer_info.key, &order.mint_a)?;
        }
        transfer_signed(
            &order_info,
            &vault_a_info,
            &beneficiary_a_info,
            &token_program,
            vault_a.amount(),
            &[&ORDER, signer_info.key.as_ref(), order.id.as_ref()],
        )?;
    }
    if vault_b.amount() > 0 {
        if beneficiary_b_info.data_is_empty() {
            create_associated_token_account(
                signer_info,
                signer_info,
                beneficiary_b_info,
                mint_b_info,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            beneficiary_b_info.as_associated_token_account(&signer_info.key, &order.mint_b)?;
        }
        transfer_signed(
            &order_info,
            &vault_b_info,
            &beneficiary_b_info,
            &token_program,
            vault_b.amount(),
            &[&ORDER, signer_info.key.as_ref(), order.id.as_ref()],
        )?;
    }

    // Close the order account.
    order_info.close(signer_info)?;

    // Close the escrow vaults.
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
