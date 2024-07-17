use sp1_helper::BuildArgs;

fn main() {
    let client_build_args = BuildArgs {
        elf_name: "riscv32im-succinct-zkvm-client-elf".to_string(),
        // docker: true,
        ignore_rust_version: true,
        ..Default::default()
    };
    sp1_helper::build_program_with_args(
        &format!("{}/../zkvm-client", env!("CARGO_MANIFEST_DIR")),
        client_build_args,
    );

    let agg_build_args = BuildArgs {
        elf_name: "riscv32im-succinct-aggregator-elf".to_string(),
        // docker: true,
        ignore_rust_version: true,
        ..Default::default()
    };
    sp1_helper::build_program_with_args(
        &format!("{}/../aggregation", env!("CARGO_MANIFEST_DIR")),
        agg_build_args,
    );
}
