# kona-sp1

Standalone repo to use Kona & SP1 to verify Optimism blocks.

## Overview

**`crates`**
- `client-utils`: A suite of utilities for the client program.
- `host-utils`: A suite of utilities for constructing the host which runs the SP1 Kona program.

**`sp1-kona`**
- `native-host`: The host program which runs the Kona program natively using `kona`.
- `zkvm-host`: The host program which runs the Kona program in SP1.
- `zkvm-client`: The program proven in SP1. The `zkvm-host` first runs `zkvm-client` on the `kona-host` to fetch the witness data, then uses SP1 to generate the program's proof of execution.

## Usage

Execute the SP1 Kona program for a single block.

```bash
just run-single <l2_block_num> [verbosity] [use-cache]
```

Execute the SP1 Kona program for a range of blocks.

```bash
just run-multi <start> <end> [verbosity] [use-cache]
```

- [verbosity]: Optional verbosity level (default: 0).
- [use-cache]: Optional flag to re-use the native execution cache (default: false).

Observations: 
* For most blocks, the cycle count per transaction is around 15M cycles per transaction.

For this block, the cycle count blows up.
* RUST_LOG=debug just run-single 122912537, 122912538
- This block fails on some cache data error.
```bash
    stderr: called `Result::unwrap()` on an `Err` value: Failed to execute pre block call: database error: Key not found in cache: 02231c1d1c3676ed8934457f53aaad3689206f5bd5cc006768e8611cb9099c6f
    stderr: stack backtrace:
    stderr: note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    2024-08-03T00:42:55.991621Z  INFO execute: close time.busy=480s time.idle=2.12µs
    thread 'main' panicked at zkvm-host/bin/multi.rs:74:10:
    called `Result::unwrap()` on an `Err` value: execution failed with exit code 1
```