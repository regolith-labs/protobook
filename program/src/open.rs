use protobook_api::prelude::*;
use spl_associated_token_account::get_associated_token_address;
use steel::*;

/// Opens an order.
pub fn process_open(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse args.
    let clock = Clock::get()?;
    let args = Open::try_from_bytes(data)?;
    let amount_a = u64::from_le_bytes(args.amount_a);
    let amount_b = u64::from_le_bytes(args.amount_b);
    let fee = u64::from_le_bytes(args.fee);
    let expires_at = i64::from_le_bytes(args.expires_at);
    let id = u64::from_le_bytes(args.id);
    if amount_a == 0 || amount_b == 0 || fee > amount_b || expires_at < clock.unix_timestamp {
        return Err(ProgramError::InvalidArgument);
    }

    // Load accounts.
    let [signer_info, fee_collector_info, mint_a_info, mint_b_info, order_info, sender_info, vault_a_info, vault_b_info, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    order_info.is_empty()?.is_writable()?.has_seeds(
        &[ORDER, signer_info.key.as_ref(), args.id.as_ref()],
        &protobook_api::ID,
    )?;
    sender_info
        .is_writable()?
        .as_associated_token_account(&signer_info.key, &mint_a_info.key)?;
    vault_a_info
        .is_writable()?
        .has_address(&get_associated_token_address(
            &order_info.key,
            &mint_a_info.key,
        ))?;
    vault_b_info
        .is_writable()?
        .has_address(&get_associated_token_address(
            &order_info.key,
            &mint_b_info.key,
        ))?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Safety check.
    assert!(mint_a_info.key != mint_b_info.key);

    // Create an order.
    create_program_account::<Order>(
        order_info,
        system_program,
        signer_info,
        &protobook_api::ID,
        &[ORDER, signer_info.key.as_ref(), args.id.as_ref()],
    )?;
    let order = order_info.as_account_mut::<Order>(&protobook_api::ID)?;
    order.authority = *signer_info.key;
    order.amount_a = amount_a;
    order.amount_b = amount_b;
    order.expires_at = expires_at;
    order.fee = fee;
    order.fee_collector = *fee_collector_info.key;
    order.id = id;
    order.mint_a = *mint_a_info.key;
    order.mint_b = *mint_b_info.key;
    order.total_deposits = 0;
    order.total_receipts = 0;
    order.total_redeemed = 0;
    order.is_collected = 0;

    // Create escrow vaults for tokens A and B.
    if vault_a_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            order_info,
            vault_a_info,
            mint_a_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }
    if vault_b_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            order_info,
            vault_b_info,
            mint_b_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    // Lock token A in escrow.
    transfer(
        signer_info,
        sender_info,
        vault_a_info,
        token_program,
        amount_a,
    )?;

    Ok(())
}
