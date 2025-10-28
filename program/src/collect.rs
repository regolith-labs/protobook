use protobook_api::prelude::*;
use steel::*;

/// Collects tokens from an expired order.
pub fn process_collect(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, fee_collector_info, mint_info, order_info, vault_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    mint_info.as_mint()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.authority == *signer_info.key)?
        .assert_mut(|o| o.expires_at < clock.unix_timestamp)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Check if order is filled.
    let is_filled = order.total_deposits == order.amount_b;

    // Validate token accounts and get transfer amount.
    let amount = if is_filled {
        mint_info.has_address(&order.mint_b)?;
        if beneficiary_info.data_is_empty() {
            create_associated_token_account(
                signer_info,
                signer_info,
                beneficiary_info,
                mint_info,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            beneficiary_info.as_associated_token_account(&signer_info.key, &order.mint_b)?;
        }
        vault_info.as_associated_token_account(&order_info.key, &order.mint_b)?;
        fee_collector_info.as_associated_token_account(&order.fee_collector, &order.mint_b)?;
        order.total_deposits - order.fee
    } else {
        mint_info.has_address(&order.mint_a)?;
        if beneficiary_info.data_is_empty() {
            create_associated_token_account(
                signer_info,
                signer_info,
                beneficiary_info,
                mint_info,
                system_program,
                token_program,
                associated_token_program,
            )?;
        } else {
            beneficiary_info.as_associated_token_account(&signer_info.key, &order.mint_a)?;
        }
        vault_info.as_associated_token_account(&order_info.key, &order.mint_a)?;
        order.amount_a
    };

    // Record the collection.
    order.is_collected = 1;

    // Withdraw from escrow to the order authority.
    transfer_signed(
        order_info,
        vault_info,
        beneficiary_info,
        token_program,
        amount,
        &[ORDER, signer_info.key.as_ref(), order.id.as_ref()],
    )?;

    // If filled, pay the fee collector.
    if is_filled && order.fee > 0 {
        transfer_signed(
            order_info,
            vault_info,
            fee_collector_info,
            token_program,
            order.fee,
            &[ORDER, signer_info.key.as_ref(), order.id.as_ref()],
        )?;
    }

    Ok(())
}
