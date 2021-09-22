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
program_id: &Pubkey, // Public key of the account the hello world program was loaded into
accounts: &[AccountInfo], // The account to say hello to
_instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
 msg!("Hello  sfsa");
    Ok(())
}
