//Building -
//cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    clock::Epoch,
};

use std::mem;

entrypoint!(process_instruction);


pub fn process_instruction(
program_id: &Pubkey, 
accounts: &[AccountInfo], 
_instruction_data: &[u8],
) -> ProgramResult {
 msg!("Hello  sfsa{}",program_id);
    Ok(())
}


pub fn invoke_process() 
-> ProgramResult {
    let program_id=Pubkey::default();
    let key=Pubkey::default();
    let mut lamports=0;
    let mut data=vec![0;mem::size_of::<u32>()];
    let owner =Pubkey::default();
    let account=AccountInfo::new(
        &key,
        false,//this is was the transaction signed or not .
        true,//can we write to this account.
        &mut lamports,//Updatable balance .
        &mut data,//updatable data.
        &owner,//progam which owns this account.
        false,//No idea.
        Epoch::default(),//Something for rent .
    );

    let instruction_data:Vec<u8>=Vec::new();
    let accounts=vec![account];


    let temp= process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    Ok(())

}
