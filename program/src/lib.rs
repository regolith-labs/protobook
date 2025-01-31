mod cancel;
mod collect;
mod close;
mod fill;
mod open;
mod redeem;

use cancel::*;
use collect::*;
use close::*;
use fill::*;
use open::*;
use redeem::*;

use protobook_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&protobook_api::ID, program_id, data)?;

    match ix {
        ProtobookInstruction::Cancel => process_cancel(accounts, data)?,
        ProtobookInstruction::Close => process_close(accounts, data)?,
        ProtobookInstruction::Collect => process_collect(accounts, data)?,
        ProtobookInstruction::Fill => process_fill(accounts, data)?,
        ProtobookInstruction::Open => process_open(accounts, data)?,
        ProtobookInstruction::Redeem => process_redeem(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
