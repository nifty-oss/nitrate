#![allow(clippy::missing_safety_doc)]

extern crate nitrate_macro;
pub use nitrate_macro::*;

pub mod program {
    pub use nitrate_program::*;
}

/// Declare the program entrypoint and set up global handlers.
///
/// The main difference from the standard `entrypoint!` macro is that this macro represents an
/// entrypoint that does not perform allocattions or copies when reading the input buffer.
///
/// This macro emits the common boilerplate necessary to begin program execution, calling a
/// provided function to process the program instruction supplied by the runtime, and reporting
/// its result to the runtime.
///
/// It also sets up a [global allocator] and [panic handler], using the [`custom_heap_default`]
/// and [`custom_panic_default`] macros from the [`solana_program`] crate.
///
/// [`custom_heap_default`]: https://docs.rs/solana-program/latest/solana_program/macro.custom_heap_default.html
/// [`custom_panic_default`]: https://docs.rs/solana-program/latest/solana_program/macro.custom_panic_default.html
/// [`solana_program`]: https://docs.rs/solana-program/latest/solana_program/index.html
///
/// The first argument is the name of a function with this type signature:
///
/// ```ignore
/// fn process_instruction(
///     program_id: &Pubkey,      // Public key of the account the program was loaded into
///     accounts: &[AccountInfo], // All accounts required to process the instruction
///     instruction_data: &[u8],  // Serialized instruction-specific data
/// ) -> ProgramResult;
/// ```
///
/// The second argument is the maximum number of accounts that the program is expecting. A program
/// can receive more than the specified maximum, but any account exceeding the maximum will be
/// ignored.
///
/// # Examples
///
/// Defining an entrypoint which reads up to 10 accounts and making it conditional on the
/// `no-entrypoint` feature. Although the `entrypoint` module is written inline in this example,
/// it is common to put it into its own file.
///
/// ```no_run
/// #[cfg(not(feature = "no-entrypoint"))]
/// pub mod entrypoint {
///
///     use nitrate::{entrypoint, program::AccountInfo};
///     use solana_program::{
///         entrypoint::ProgramResult,
///         msg,
///         pubkey::Pubkey,
///     };
///
///     entrypoint!(process_instruction, 10);
///
///     pub fn process_instruction(
///         program_id: &Pubkey,
///         accounts: &[AccountInfo],
///         instruction_data: &[u8],
///     ) -> ProgramResult {
///         msg!("Hello from my program!");
///
///         Ok(())
///     }
///
/// }
/// ```
#[macro_export]
macro_rules! entrypoint {
    ( $process_instruction:ident, $maximum:literal ) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            // create an array of uninitialized account infos; it is safe to `assume_init` since
            // we are claiming that the aray of `MaybeUninit` is initialized and `MaybeUniint` do
            // not require initialization
            let mut accounts: [std::mem::MaybeUninit<$crate::program::AccountInfo>; $maximum] =
                std::mem::MaybeUninit::uninit().assume_init();

            let (program_id, count, instruction_data) =
                $crate::program::deserialize::<$maximum>(input, accounts.as_mut_ptr());

            // call the program's entrypoint passing `count` account infos; we know that
            // they are initialized so we cast the pointer to a slice of `[AccountInfo]`
            match $process_instruction(
                &program_id,
                std::slice::from_raw_parts(accounts.as_ptr() as _, count),
                &instruction_data,
            ) {
                Ok(()) => solana_program::entrypoint::SUCCESS,
                Err(error) => error.into(),
            }
        }

        solana_program::custom_heap_default!();
        solana_program::custom_panic_default!();
    };
}
