set fallback := true
set dotenv-load

# default recipe to display help information
default:
  @just --list

run l2_block_num verbosity="0":
  cargo run --bin single_block --release -- --l2-block-number {{l2_block_num}} --verbosity {{verbosity}}
