use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::{find_program_address, Pubkey}};
use pinocchio_system::ID;
use pinocchio_token::{state::Mint, ID};

use crate::Crowdfund::Crowdfund;

pub trait AccountCheck{
    fn check(account:&AccountInfo)->Result<(),ProgramError>;
}
pub struct MintAccount;
impl AccountCheck for MintAccount{
    fn check(account:&AccountInfo)->Result<(),ProgramError> {
        if !account.is_owned_by(&ID){
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_len()!=Mint::LEN{
            return Err(ProgramError::InvalidAccountData);
        }
Ok(())
    }
}

pub struct CrowfundAccount{}

impl AccountCheck for CrowfundAccount{
    fn check(account:&AccountInfo)->Result<(),ProgramError> {
        if !account.is_owned_by(&ID){
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_len()!=Crowdfund::INIT_SPACE{
            return Err(ProgramError::InvalidAccountData);
        }
        let seeds=[
            Seed::from(b"crowdfund"),
        ];

        let signer_seeds=Signer::from(&seeds);
        let CrowdfundAccount=find_program_address(&[b"crowdfund"], &ID);
        if account.key().ne(&CrowdfundAccount.0){
            return Err(ProgramError::InvalidAccountOwner);
        }
Ok(())
    }
}