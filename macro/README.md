# Macro

Companion macro for programs using [`nitrate`](https://github.com/nifty-oss/nitrate) entrypoint. It creates structs to provide an easy way to access instruction accounts, while being more efficient than using `solana_program::account_info::next_account_info` iterator.

## Getting started

The macro is part of [`nitrate`](https://github.com/nifty-oss/nitrate) crate:

```bash
cargo add nitrate
```

> [!IMPORTANT]
> You need to use the custom `entrypoint!` defined on the crate in order to use the `Accounts` derive macro.

## Example

Annotate your instruction enum with `Accounts` derive:

```rust
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankInstruction, Accounts)]
pub struct Instruction {
    #[account(0, signer, writable, name="buffer", desc = "The unitialized buffer account")]
    #[account(1, writable, name="recipient", desc = "The account receiving refunded rent")]
    Close,

    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, writable, name="signer", desc = "The owner or burn delegate of the asset")]
    #[account(2, optional, writable, name="recipient", desc = "The account receiving refunded rent")]
    #[account(3, optional, writable, name="group", desc = "Asset account of the group")]
    Burn,
}
```

This will create a module `accounts` with a struct for each variant (instruction) of the enum:

```rust
use nitrate::program::AccountInfo;

mod accounts {
    pub struct Close<'a> {
        pub buffer: &'a AccountInfo,
        pub recipient: &'a AccountInfo,
    }

    pub struct Burn<'a> {
        pub asset: &'a AccountInfo,
        pub signer: &'a AccountInfo,
        pub recipient: Option<&'a AccountInfo>,
        pub group: Option<&'a AccountInfo>,
    }
}
```

In your instruction processor, a `Context` can then be created to access the accounts of an instruction:

```rust
let ctx = Burn::context(accounts)?;
msg!("Burn asset: {:?}", ctx.accounts.asset.key());
```

## License

Copyright (c) 2024 nifty-oss maintainers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
