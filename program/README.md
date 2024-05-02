# Program

Types and helper functions for programs using [`nitrate`](https://github.com/nifty-oss/nitrate) entrypoint.

* `account_info`: Account representation.
* `cpi`: Helper types to create cross-program invocations using `sol_invoke_signed_c`.
* `system`: Helper functions to invoke `solana_program::system_program`.

## Getting started

The types are used as part of the [`nitrate`](https://github.com/nifty-oss/nitrate) crate:

```bash
cargo add nitrate
```

> [!IMPORTANT]
> You need to use the custom `entrypoint!` defined on the crate in order to use the types included in this crate.

## License

Copyright (c) 2024 nifty-oss maintainers
Copyright (c) 2024 Magnetar Fields

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

This crate is based on [`solana-nostd-entrypoint`](https://github.com/cavemanloverboy/solana-nostd-entrypoint/tree/main).
