use protobook_api::prelude::*;
use steel::*;

/// Fills an order.
pub fn process_fill(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = Fill::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, order_info, receipt_info, sender_info, vault_b_info, system_program, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let order = order_info
        .as_account_mut::<Order>(&protobook_api::ID)?
        .assert_mut(|o| o.expires_at > clock.unix_timestamp)?
        .assert_mut(|o| o.amount_b > o.total_deposits)?;
    receipt_info.is_writable()?.has_seeds(
        &[RECEIPT, signer_info.key.as_ref(), order_info.key.as_ref()],
        &protobook_api::ID,
    )?;
    vault_b_info
        .is_writable()?
        .as_associated_token_account(&order_info.key, &order.mint_b)?;
    sender_info
        .is_writable()?
        .as_associated_token_account(&signer_info.key, &order.mint_b)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Create receipt account, if necessary.
    let receipt = if receipt_info.data_is_empty() {
        create_program_account::<Receipt>(
            receipt_info,
            system_program,
            signer_info,
            &protobook_api::ID,
            &[RECEIPT, signer_info.key.as_ref(), order_info.key.as_ref()],
        )?;
        let receipt = receipt_info.as_account_mut::<Receipt>(&protobook_api::ID)?;
        receipt.authority = *signer_info.key;
        receipt.deposit = 0;
        receipt.order = *order_info.key;
        receipt
    } else {
        receipt_info.as_account_mut::<Receipt>(&protobook_api::ID)?
    };

    // Lock token B in escrow.
    let remaining = order.amount_b - order.total_deposits;
    let amount = amount.min(remaining);
    transfer(signer_info, sender_info, order_info, vault_b_info, amount)?;

    // Record the deposit.
    receipt.deposit += amount;
    order.total_deposits += amount;
    order.total_receipts += 1;

    // If filled, expire the order immediately.
    if order.total_deposits == order.amount_b {
        order.expires_at = clock.unix_timestamp;
    }

    Ok(())
}
