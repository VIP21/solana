//! @brief Example Rust-based BPF program that tests duplicate accounts passed via accounts

extern crate solana_sdk;
use solana_sdk::{
    account_info::AccountInfo, entrypoint, entrypoint::SUCCESS, info, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(_program_id: &Pubkey, accounts: &mut [AccountInfo], data: &[u8]) -> u32 {
    const FAILURE: u32 = 1;

    match data[0] {
        1 => {
            info!("modify first account data");
            accounts[2].data[0] = 1;
        }
        2 => {
            info!("modify first account data");
            accounts[3].data[0] = 2;
        }
        3 => {
            info!("modify both account data, should fail");
            accounts[2].data[0] = 1;
            accounts[3].data[0] = 2;
        }
        4 => {
            info!("modify first account lamports");
            *accounts[1].lamports -= 1;
            *accounts[2].lamports += 1;
        }
        5 => {
            info!("modify first account lamports");
            *accounts[1].lamports -= 2;
            *accounts[3].lamports += 2;
        }
        6 => {
            info!("modify both account lamports, should fail");
            *accounts[1].lamports -= 1;
            *accounts[2].lamports += 1;
            *accounts[3].lamports += 2;
        }
        _ => {
            info!("Unrecognized command");
            return FAILURE;
        }
    }
    SUCCESS
}
