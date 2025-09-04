use pinocchio::{entrypoint, ProgramResult};
pub mod state;
pub use state::Crowdfund;
pub mod instructions;
pub use instructions::*;
pub mod helpers;
pub use  helpers::*;

entrypoint!(process_instruction);

fn process_instruction()->ProgramResult{

}

