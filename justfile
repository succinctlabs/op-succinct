set fallback := true
set dotenv-load

# default recipe to display help information
default:
    @just --list

run l2_block_num:
    RUST_LOG=info cargo run --bin single_block --release -- --l2-claim-block {{l2_block_num}} --run-native

run-zkvm l2_block_num:
    RUST_LOG=info cargo run --bin single_block --release -- --l2-claim-block {{l2_block_num}}

run-multiblock start_block end_block:
    RUST_LOG=info cargo run --bin multi_block --release -- --start-block {{start_block}} --end-block {{end_block}} --run-native

estimate-cost start_block end_block l2_rpc skip_datagen verbosity:
    #!/usr/bin/env bash
    if [ -z "$(skip_datagen)" ]; then
        RUST_LOG=info cargo run --bin cost_estimator --release -- --start-block {{start_block}} --end-block {{end_block}} --rpc-url {{l2_rpc}} --skip-datagen --verbosity-level {{verbosity}}
    else
        RUST_LOG=info cargo run --bin cost_estimator --release -- --start-block {{start_block}} --end-block {{end_block}} --rpc-url {{l2_rpc}} --verbosity-level {{verbosity}}
    fi
