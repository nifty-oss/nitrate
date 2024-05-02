<h1 align="center">
  <code>nitrate</code>
</h1>
<p align="center">
  <img width="400" alt="Nitrate" src="https://github.com/nifty-oss/nitrate/assets/729235/98c2f3cb-054b-4bbc-9c85-c89db0a7e74a" />
</p>

<p align="center">
  A custom lightweight entrypoint for Solana programs.
</p>

<p align="center">
  <a href="https://crates.io/crates/nitrate"><img src="https://img.shields.io/crates/v/nitrate?logo=rust" /></a>
</p>

## Getting started

From your project folder:

```bash
cargo add nitrate
```

On your entrypoint definition:
```rust
use nitrate::{entrypoint, program::AccountInfo};
use solana_program::{
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// custom entrypoint definition:
//
//   1. name of the process instruction function
//   2. expected number of accounts
entrypoint!(process_instruction, 10);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from my program!");

    Ok(())
}
```

The main difference from the standard `entrypoint!` macro is that `nitrate` represents an entrypoint that does not perform allocations or copies when reading the input buffer, and therefore uses less compute units to parse the input accounts.

The entrypoint is bundled with a companion [`program`](https://github.com/nifty-oss/nitrate/program/README.md) types and [`macro`](https://github.com/nifty-oss/nitrate/macro/README.md) crates.

> [!TIP]
> A program can receive more than the specified maximum number of accounts, but any account exceeding the maximum will be ignored. On an ideal scenario, this number should be equal to the number of accounts required by the largest instruction of your program.

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

This crate is based on [`solana-nostd-entrypoint`](https://github.com/cavemanloverboy/solana-nostd-entrypoint) under the [Apache-2.0 license](./LICENSE.third-party).
