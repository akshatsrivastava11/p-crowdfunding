use pinocchio::{account_info::AccountInfo, program_error::ProgramError};
use pinocchio_system::ID;
use pinocchio_token::state::Mint;

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
