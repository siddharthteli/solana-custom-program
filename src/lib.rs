//Building -
//cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);


pub fn process_instruction(
program_id: &Pubkey, 
accounts: &[AccountInfo], 
_instruction_data: &[u8],
) -> ProgramResult {
 msg!("Hello  sfsa{}",program_id);
    Ok(())
}
