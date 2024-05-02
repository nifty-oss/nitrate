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

pub mod account_info;
pub mod cpi;
pub mod system;

pub use account_info::*;

use solana_program::{
    entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE, NON_DUP_MARKER},
    pubkey::Pubkey,
};
use std::slice::from_raw_parts;

/// Deserialize the input arguments.
///
/// This can only be called from the entrypoint function of a Solana program and with
/// a buffer that was serialized by the runtime.
#[allow(clippy::cast_ptr_alignment, clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn deserialize<'a, const MAX_ACCOUNTS: usize>(
    input: *mut u8,
    accounts: *mut std::mem::MaybeUninit<AccountInfo>,
) -> (&'a Pubkey, usize, &'a [u8]) {
    let mut offset: usize = 0;

    // total number of accounts present; it only process up to MAX_ACCOUNTS
    let total_accounts = *(input.add(offset) as *const u64) as usize;

    // number of processed accounts
    let count = if total_accounts <= MAX_ACCOUNTS {
        total_accounts
    } else {
        #[cfg(feature = "logging")]
        solana_program::log::sol_log("🟡 Number of accounts exceeds MAX_ACCOUNTS");

        MAX_ACCOUNTS
    };

    offset += std::mem::size_of::<u64>();

    for i in 0..count {
        let duplicate_info = *(input.add(offset) as *const u8);
        if duplicate_info == NON_DUP_MARKER {
            // MAGNETAR FIELDS: safety depends on alignment, size
            // 1) we will always be 8 byte aligned due to align_offset
            // 2) solana vm serialization format is consistent so size is ok
            let account_info: *mut Account = input.add(offset) as *mut _;

            offset += std::mem::size_of::<Account>();
            offset += (*account_info).data_len as usize;
            offset += MAX_PERMITTED_DATA_INCREASE;
            offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128);
            offset += std::mem::size_of::<u64>(); // MAGNETAR FIELDS: ignore rent epoch

            // MAGNETAR FIELDS: reset borrow state right before pushing
            (*account_info).borrow_state = 0b_0000_0000;

            std::ptr::write(
                accounts.add(i),
                std::mem::MaybeUninit::new(AccountInfo {
                    raw: account_info as *const _ as *mut _,
                }),
            );
        } else {
            offset += 8;
            // duplicate account, clone the original
            std::ptr::copy_nonoverlapping(
                accounts.add(duplicate_info as usize),
                accounts.add(i),
                1,
            );
        }
    }

    // process any remaining accounts to move the offset to the instruction
    // data (there is a duplication of logic but we avoid testing whether we
    // have space for the account or not)
    for _ in count..total_accounts {
        let duplicate_info = *(input.add(offset) as *const u8);

        if duplicate_info == NON_DUP_MARKER {
            let account_info: *mut Account = input.add(offset) as *mut _;
            offset += std::mem::size_of::<Account>();
            offset += (*account_info).data_len as usize;
            offset += MAX_PERMITTED_DATA_INCREASE;
            offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128);
            offset += std::mem::size_of::<u64>(); // MAGNETAR FIELDS: ignore rent epoch
        } else {
            offset += 8;
        }
    }

    // instruction data
    #[allow(clippy::cast_ptr_alignment)]
    let instruction_data_len = *(input.add(offset) as *const u64) as usize;
    offset += std::mem::size_of::<u64>();

    let instruction_data = { from_raw_parts(input.add(offset), instruction_data_len) };
    offset += instruction_data_len;

    // program id
    let program_id: &Pubkey = &*(input.add(offset) as *const Pubkey);

    (program_id, count, instruction_data)
}
