set fallback := true
set dotenv-load

# default recipe to display help information
default:
  @just --list

run l2_block_num:
  cargo run --bin script --release -- --l2_block_num {{l2_block_num}} --run-native

run-zkvm l2_block_num:
  cargo run --bin script --release -- l2_block_num={{l2_block_num}}

estimate-cost start_block end_block l2_rpc skip_datagen verbosity:
  if [ -z "$(skip_datagen)" ]; then
    cargo run --bin cost_estimator --release -- --start-block {{start_block}} --end-block {{end_block}} --rpc-url {{l2_rpc}} --skip-datagen --verbosity-level {{verbosity}}
  else
    cargo run --bin cost_estimator --release -- --start-block {{start_block}} --end-block {{end_block}} --rpc-url {{l2_rpc}} --verbosity-level {{verbosity}}
