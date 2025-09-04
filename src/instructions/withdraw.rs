use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::{AccountCheck, CrowfundAccount};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};
pub struct Withdraw{}
impl Withdraw {
    fn process(accounts:&[AccountInfo],instruction_data:&[u8])->ProgramResult{
        let [host,
            crowdfund,
            vault,
            host_ata_token]=accounts else{
                return  Err(ProgramError::InvalidAccountData);
            };
        if !host.is_signer(){
            return  Err(ProgramError::MissingRequiredSignature);
        };
        CrowfundAccount::check(crowdfund);
        let signers=Signer::from(&[Seed::from(b"crowdfund")]);
        Transfer{
            from:vault,
            to:host_ata_token,
            amount:TokenAccount::from_account_info(vault)?.amount(),
            authority:host,
        }.invoke_signed(signers);
        
    }
}