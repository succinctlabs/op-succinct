set fallback := true
set dotenv-load

# default recipe to display help information
default:
  @just --list

run l2_block_num
  cargo run --bin script --release -- --l2_block_num {{l2_block_num}} --run-native

run-zkvm l2_block_num:
  cargo run --bin script --release -- l2_block_num={{l2_block_num}}

estimate-cost {{start_block}} {{end_block}} {{l2_rpc}} {{skip_datagen}} {{verbosity}}:
  cargo run --bin cost-estimator --release -- --start-block {{start_block}} --end-block {{end_block}} --l2-rpc {{l2_rpc}} --skip-datagen {{skip_datagen}} --verbosity-level {{verbosity}}
