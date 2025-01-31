use protobook_api::prelude::*;
use steel::*;

/// Redeems a receipt for an expired order.
pub fn process_redeem(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, beneficiary_info, order_info, receipt_info, vault_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.expires_at < clock.unix_timestamp)?;
    let receipt = receipt_info
        .as_account_mut::<Receipt>(&protobook_api::ID)?
        .assert_mut(|r| r.authority == *signer_info.key)?;
    token_program.is_program(&spl_token::ID)?;

    // Check if order is filled.
    let is_filled = order.total_deposits == order.amount_b;
    
    // Validate token accounts and get transfer amount.
    let amount = if is_filled {
        beneficiary_info.as_associated_token_account(&signer_info.key, &order.mint_a)?;
        vault_info.as_associated_token_account(&order_info.key, &order.mint_a)?;
        order.amount_a * receipt.deposit / order.total_deposits
    } else {
        beneficiary_info.as_associated_token_account(&signer_info.key, &order.mint_b)?;
        vault_info.as_associated_token_account(&order_info.key, &order.mint_b)?;
        receipt.deposit
    };

    // Withdraw from escrow to the receipt authority.
    transfer_signed(
        order_info,
        vault_info,
        beneficiary_info,
        token_program,
        amount,
        &[ORDER, signer_info.key.as_ref(), order.id.as_ref()],
    )?;

    Ok(())
}
