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

//! Cross-program invocation helper types.

use solana_program::pubkey::Pubkey;

use crate::account_info::AccountInfo;

/// An `AccountMeta`` as expected by `sol_invoke_signed_c`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CAccountMeta {
    // Public key of the account.
    pub pubkey: *const Pubkey,

    // Is the account writable?
    pub is_writable: bool,

    // Transaction was signed by this account's key?
    pub is_signer: bool,
}

impl From<&AccountInfo> for CAccountMeta {
    fn from(account: &AccountInfo) -> Self {
        CAccountMeta {
            pubkey: offset(account.raw, 8),
            is_writable: account.is_writable(),
            is_signer: account.is_signer(),
        }
    }
}

/// An `AccountInfo`` as expected by `sol_invoke_signed_c`.
#[repr(C)]
#[derive(Clone)]
pub struct CAccountInfo {
    // Public key of the account.
    pub key: *const Pubkey,

    // Number of lamports owned by this account.
    pub lamports: *const u64,

    // Length of data in bytes.
    pub data_len: u64,

    // On-chain data within this account.
    pub data: *const u8,

    // Program that owns this account.
    pub owner: *const Pubkey,

    // The epoch at which this account will next owe rent.
    pub rent_epoch: u64,

    // Transaction was signed by this account's key?
    pub is_signer: bool,

    // Is the account writable?
    pub is_writable: bool,

    // This account's data contains a loaded program (and is now read-only).
    pub executable: bool,
}

#[inline(always)]
const fn offset<T, U>(ptr: *const T, offset: usize) -> *const U {
    unsafe { (ptr as *const u8).add(offset) as *const U }
}

impl From<&AccountInfo> for CAccountInfo {
    fn from(account: &AccountInfo) -> Self {
        CAccountInfo {
            key: offset(account.raw, 8),
            lamports: offset(account.raw, 72),
            data_len: account.data_len() as u64,
            data: offset(account.raw, 88),
            owner: offset(account.raw, 40),
            rent_epoch: 0,
            is_signer: account.is_signer(),
            is_writable: account.is_writable(),
            executable: account.executable(),
        }
    }
}

/*
impl CAccountInfo {
    /// A CPI utility function
    #[inline(always)]
    pub(crate) fn to_account_meta(&self) -> CAccountMeta {
        CAccountMeta {
            pubkey: self.key,
            is_writable: self.is_writable,
            is_signer: self.is_signer,
        }
    }

    /// A CPI utility function.
    /// Intended for PDAs that didn't sign transaction but must sign for cpi.
    #[inline(always)]
    pub(crate) fn to_account_meta_signer(&self) -> CAccountMeta {
        CAccountMeta {
            pubkey: self.key,
            is_writable: self.is_writable,
            is_signer: true,
        }
    }
}
*/

/// An `Instruction` as expected by `sol_invoke_signed_c`.
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct CInstruction {
    /// Public key of the program.
    pub program_id: *const Pubkey,

    /// Accounts expected by the program instruction.
    pub accounts: *const CAccountMeta,

    /// Number of accounts expected by the program instruction.
    pub accounts_len: u64,

    /// Data expected by the program instruction.
    pub data: *const u8,

    /// Length of the data expected by the program instruction.
    pub data_len: u64,
}

/// A signer seed as expected by `sol_invoke_signed_c`.
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct CSignerSeed {
    /// Seed bytes.
    pub seed: *const u8,

    /// Length of the seed bytes.
    pub len: u64,
}

/// Signer as expected by `sol_invoke_signed_c`.
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct CSigner {
    /// Seed bytes.
    pub seeds: *const CSignerSeed,

    /// Number of signers.
    pub len: u64,
}
