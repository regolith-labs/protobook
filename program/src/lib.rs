mod cancel;
mod claim;
mod close;
mod collect;
mod expire;
mod fill;
mod open;
mod withdraw;

use cancel::*;
use claim::*;
use close::*;
use collect::*;
use expire::*;
use fill::*;
use open::*;
use withdraw::*;

use protobook_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&protobook_api::ID, program_id, data)?;

    match ix {
        // Makers
        ProtobookInstruction::Cancel => todo!(), // process_cancel(accounts, data)?,
        ProtobookInstruction::Claim => todo!(), // process_claim(accounts, data)?,
        ProtobookInstruction::Close => todo!(), // process_close(accounts, data)?,
        ProtobookInstruction::Expire => todo!(), // process_expire(accounts, data)?,
        ProtobookInstruction::Open => process_open(accounts, data)?,

        // Takers
        ProtobookInstruction::Collect => todo!(), // process_collect(accounts, data)?,
        ProtobookInstruction::Fill => todo!(), // process_fill(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
