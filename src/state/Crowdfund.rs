use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub struct Crowdfund{
    pub host:Pubkey,
    pub amount_neeeded:[u8;1],
    pub amount_collected:[u8;1],
    pub mint_to_raise:Pubkey,
    pub end_time:u64
}
impl Crowdfund{
    pub const INIT_SPACE:usize=8+32+8+8+32;
    pub fn load_mut(bytes:&mut [u8])->Result<&mut Self,ProgramError>{
        if bytes.len()!=Crowdfund::INIT_SPACE{
            return  Err(ProgramError::InvalidAccountData);
        }
        Ok(
            unsafe {
                &mut *(bytes.as_mut_ptr() as *mut  Crowdfund)
            
            }
        )
    }

    #[inline(always)]
    pub fn set_host(&mut self,host:Pubkey){
        self.host=host;
    }

    #[inline(always)]
    pub fn set_amount_needed(&mut self,amount_needed: [u8;1]){
        self.amount_neeeded=amount_needed;
    }


    #[inline(always)]
    pub fn  set_amount_collected(&mut self,amount_collected:[u8;1]){
        self.amount_collected=amount_collected;
    }


    #[inline(always)]
    pub fn set_mint_to_raise(&mut self,mint_to_raise:Pubkey){
        self.mint_to_raise=mint_to_raise;
    }

    #[inline(always)]
    pub fn set_end_time(&mut self,end_time:u64){
        self.end_time=end_time;
    }

    #[inline(always)]
    pub fn set_inner(&mut self,host:&Pubkey,amount_needed:&[u8;1],amount_collected:&[u8;1],mint_to_raise:&Pubkey,end_time:&u64){
        self.amount_collected=*amount_collected;
        self.amount_neeeded=*amount_needed;
        self.host=*host;
        self.mint_to_raise=*mint_to_raise;
        self.end_time=*end_time;
    }

}