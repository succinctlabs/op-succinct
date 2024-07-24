# kona-sp1

Standalone repo to use Kona & SP1 to verify Optimism blocks.

## Usage

To run the program in witness gen mode, then use that witness to run the execution of the proof in the zkVM.

```bash
just run-single [l2_block_num]
```

## Single Block Executor

Execute a single block program.

```bash
cargo run --bin single -- --l2-block <L2_BLOCK_NUMBER>
```

- Optional: `--use-cache` to re-use the cache. `--verbosity` <LEVEL> to set verbosity level.

Output:
```

```
