// Main entry point for the Solana program
// Responsible for:
// - Program initialization
// - Instruction routing
// - Module organization

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

pub mod processor;
pub mod instruction;
pub mod state;
pub mod error;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction");
    processor::process(program_id, accounts, instruction_data)
}
