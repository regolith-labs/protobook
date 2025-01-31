use protobook_api::prelude::*;
use steel::*;

pub fn process_open(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = Open::try_from_bytes(data)?;

    // Load accounts.
    let [signer_info, mint_a_info, mint_b_info, order_info, sender_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);        
    };
    signer_info.is_signer()?;
    mint_a_info.as_mint()?;
    mint_b_info.as_mint()?;
    order_info.is_empty()?.is_writable()?.has_seeds(
        &[ORDER, signer_info.key.as_ref(), args.seed.as_ref()],
        &protobook_api::ID
    )?;
    sender_info
        .is_writable()?
        .as_associated_token_account(&signer_info.key, &mint_a_info.key)?;
    system_program.is_program(&system_program::ID)?;

    // Initialize counter.
    create_account::<Order>(
        order_info,
        system_program,
        signer_info,
        &protobook_api::ID,
        &[ORDER, signer_info.key.as_ref(), args.seed.as_ref()],
    )?;
    let order = order_info.as_account_mut::<Order>(&protobook_api::ID)?;
    order.authority = *signer_info.key;
    order.amount_a = args.amount_a;
    order.amount_b = args.amount_b;
    order.commission = args.commission;
    order.commission_recipient = Pubkey::default(); // TODO
    order.expires_at = args.expires_at;
    order.filled = 0;
    order.mint_a = *mint_a_info.key;
    order.mint_b = *mint_b_info.key;
    order.seed = args.seed;
    order.threshold = args.threshold;
    order.total_receipts = 0;

    Ok(())
}
