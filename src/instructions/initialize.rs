use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, sysvars::{rent::Rent, Sysvar}, ProgramResult};
use pinocchio_system::{instructions::CreateAccount, ID};

use crate::{AccountCheck, Crowdfund::Crowdfund, MintAccount};
 
pub struct Initialize{}
impl Initialize{
    pub fn process(accounts:&[AccountInfo],instruction_data:&[u8])->ProgramResult{
        let [host,
     mint_to_raise,
     crowdfund
     ]=accounts else{
        return  Err(ProgramError::InvalidAccountData);
     };
    //  let [amount_needed,end_time]=instruction_data else{
    //     return  Err(ProgramError::InvalidAccountData);
    //  };
let amount_needed = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
let end_time = u64::from_le_bytes(instruction_data[8..16].try_into().unwrap());

     if !host.is_signer(){
        return  Err(ProgramError::MissingRequiredSignature);
     };
     if amount_needed==0  {
        return Err(ProgramError::InvalidInstructionData);
     };
     MintAccount::check(mint_to_raise);
     let crowdfund_seeds=[
        Seed::from(b"crowdfund"),
     ]; 
     let signers=Signer::from(&crowdfund_seeds);
     CreateAccount{
        from:host,
        to:crowdfund,
        owner:&ID,
        lamports:Rent::get()?.minimum_balance(Crowdfund::INIT_SPACE),
        space:Crowdfund::INIT_SPACE as u64,
     }.invoke_signed(&[signers]);
     let mut bindings=crowdfund.try_borrow_mut_data()?;
     let crowdfund_Data:&mut Crowdfund=Crowdfund::load_mut(&mut bindings).unwrap();
    crowdfund_Data.set_inner(
    &host.key(),
    &[amount_needed as u8],
    &[0 as u8],
    &mint_to_raise.key(),
    &end_time,
);


     Ok(())

    }
}
