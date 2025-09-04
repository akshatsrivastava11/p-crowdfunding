use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};

use crate::{helpers, AccountCheck, Crowdfund::Crowdfund, CrowfundAccount};
use pinocchio_token::instructions::Transfer;
pub struct Contribute{}

impl Contribute{
    //send spl-token over to the vault account
    pub fn process(accounts:&[AccountInfo],instruction_data:&[u8])->ProgramResult{
        let [host,
            crowdfund,
            host_ata_token  ,
            vault
            ]=accounts else{
                return Err(ProgramError::InvalidAccountData);
            };
            let amount=u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

        if !host.is_signer(){
            return Err(ProgramError::MissingRequiredSignature);
        };

        CrowfundAccount::check(crowdfund);
        let mut bindings=crowdfund.try_borrow_mut_data()?;
        let crowdfund_Data:&mut Crowdfund=Crowdfund::load_mut(&mut bindings).unwrap();
        Transfer{
            from:host_ata_token,
            to:vault,
            amount,
            authority:host,
        }.invoke();
        crowdfund_Data.amount_collected+=amount;
        crowdfund_Data.amount_neeeded-=amount;
        msg!("Thanks for donating!!");
        Ok(())
    }
}