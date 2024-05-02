mod accounts;
use accounts::generate_accounts;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Annotates an enum with the `#[derive(Accounts)]` to derive instruction structs
/// for each variant containing the accounts for the instruction.
///
/// This macro works in conjunction with the [Shank]'s `#[account]` attribute macro
/// to define the accounts. It will create a struct for each variant of the enum that
/// can be used to create a `Context` to easily access the accounts by their name.
///
/// Using a `Context` is more efficient than using the `next_account_info` iterator
/// since the accounts structs encapsulate the order of accounts and they can be
/// indexed directly.
///
/// [Shank]: https://github.com/metaplex-foundation/shank
///
/// # Examples
///
/// ```no_run
/// #[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankInstruction, Accounts)]
/// pub struct Instruction {
///     /// Closes an uninitialized asset (buffer) account.
///     ///
///     /// You can only close the buffer account if it has not been used to create an asset.
///     #[account(0, signer, writable, name="buffer", desc = "The unitialized buffer account")]
///     #[account(1, writable, name="recipient", desc = "The account receiving refunded rent")]
///     Close,
///
///     /// Burns an asset.
///     #[account(0, writable, name="asset", desc = "Asset account")]
///     #[account(1, signer, writable, name="signer", desc = "The owner or burn delegate of the asset")]
///     #[account(2, optional, writable, name="recipient", desc = "The account receiving refunded rent")]
///     #[account(3, optional, writable, name="group", desc = "Asset account of the group")]
///     Burn,
/// }
/// ```
///
/// This will create a module `accounts` with a struct for each variant of the enum:
/// ```no_run
/// use nitrate::program::AccountInfo;
///
/// pub struct Close<'a> {
///     pub buffer: &'a AccountInfo,
///     pub recipient: &'a AccountInfo,
/// }
///
/// pub struct Burn<'a> {
///     pub asset: &'a AccountInfo,
///     pub signer: &'a AccountInfo,
///     pub recipient: Option<&'a AccountInfo>,
///     pub group: Option<&'a AccountInfo>,
/// }
/// ```
/// A `Context` can then be created to access the accounts of an instruction:
/// ```no_run
/// let ctx = Burn::context(accounts)?;
/// msg!("Burn asset: {:?}", ctx.accounts.asset.key());
/// ```
#[proc_macro_derive(Accounts, attributes(account))]
pub fn context_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let accounts = generate_accounts(ast);

    match accounts {
        Ok(accounts) => TokenStream::from(quote! {
            pub mod accounts {
                pub struct Context<T> {
                    pub accounts: T,
                }

                #accounts
            }
        }),
        Err(error) => error.to_compile_error().into(),
    }
}
