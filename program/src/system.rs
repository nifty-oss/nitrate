// Copyright (c) 2024 nifty-oss maintainers
// Copyright (c) 2024 Magnetar Fields
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! System Program CPI functions.

use solana_program::{pubkey::Pubkey, system_program};

use crate::{
    cpi::{CAccountInfo, CAccountMeta, CInstruction, CSigner, CSignerSeed},
    AccountInfo,
};

/// Create a new account.
///
/// # Arguments
///
/// * `funder`: Funding account.
/// * `account`: New account.
/// * `lamports`: Number of lamports to transfer to the new account.
/// * `space`: Number of bytes of memory to allocate.
/// * `owner`: Address of program that will own the new account.
pub fn create_account(
    funder: &AccountInfo,
    account: &AccountInfo,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) {
    _create_account_signed::<0>(funder, account, lamports, space, owner, &[]);
}

/// Create a new account with a program signed instruction.
///
/// # Arguments
///
/// * `funder`: Funding account.
/// * `account`: New account.
/// * `lamports`: Number of lamports to transfer to the new account.
/// * `space`: Number of bytes of memory to allocate.
/// * `owner`: Address of program that will own the new account.
/// * `signer_seeds`: Seeds used to sign the instruction.
pub fn create_account_signed<const SEEDS: usize>(
    funder: &AccountInfo,
    account: &AccountInfo,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
    signer_seeds: &[&[u8]; SEEDS],
) {
    // signer seeds
    let mut seeds: [std::mem::MaybeUninit<CSignerSeed>; SEEDS] =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    signer_seeds.iter().enumerate().for_each(|(i, seed)| {
        seeds[i] = std::mem::MaybeUninit::new(CSignerSeed {
            seed: seed.as_ptr(),
            len: seed.len() as u64,
        })
    });

    let signer = [CSigner {
        seeds: seeds.as_ptr() as *const CSignerSeed,
        len: SEEDS as u64,
    }];

    _create_account_signed::<SEEDS>(funder, account, lamports, space, owner, &signer);
}

/// Transfer lamports between accounts.
///
/// # Arguments
///
/// * `from`: Funding account.
/// * `recipient`: Recipient account.
/// * `amount`: Number of lamports to transfer.
pub fn transfer(from: &AccountInfo, recipient: &AccountInfo, amount: u64) {
    let instruction_accounts: [CAccountMeta; 2] = [from.into(), recipient.into()];

    // -   0..4: instruction discriminator
    // -  4..12: lamports amount
    let mut instruction_data = [0; 12];
    // transfer instruction has a '2' discriminator
    instruction_data[0] = 2;
    instruction_data[4..12].copy_from_slice(&amount.to_le_bytes());

    let instruction = CInstruction {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    // account infos and seeds
    let account_infos: [CAccountInfo; 2] = [from.into(), recipient.into()];
    let seeds: &[&[&[u8]]] = &[];

    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const CInstruction as *const u8,
            account_infos.as_ptr() as *const u8,
            account_infos.len() as u64,
            seeds.as_ptr() as *const u8,
            seeds.len() as u64,
        );
    }

    // keep clippy happy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &account_infos, &seeds));
}

//-- Internal functions

/// Create a new account.
///
/// This function is used to create a new account either with or without a program
/// signed instruction.
///
/// # Arguments
///
/// * `funder`: Funding account.
/// * `account`: New account.
/// * `lamports`: Number of lamports to transfer to the new account.
/// * `space`: Number of bytes of memory to allocate.
/// * `owner`: Address of program that will own the new account.
/// * `signer`: Seeds used to sign the instruction.
fn _create_account_signed<const SEEDS: usize>(
    funder: &AccountInfo,
    account: &AccountInfo,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
    signer: &[CSigner],
) {
    let mut instruction_accounts: [CAccountMeta; 2] = [funder.into(), account.into()];
    // account being created is always a signer
    instruction_accounts[1].is_signer = true;

    // -   0..4: instruction discriminator
    // -  4..12: lamports
    // - 12..20: account space
    // - 20..52: owner pubkey
    let mut instruction_data = [0; 52];
    // create account instruction has a '0' discriminator
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&space.to_le_bytes());
    instruction_data[20..52].copy_from_slice(owner.as_ref());

    let instruction = CInstruction {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    // account infos
    let account_infos: [CAccountInfo; 2] = [funder.into(), account.into()];

    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const CInstruction as *const u8,
            account_infos.as_ptr() as *const u8,
            account_infos.len() as u64,
            signer.as_ptr() as *const u8,
            signer.len() as u64,
        );
    }

    // keep clippy happy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &account_infos, &signer));
}
