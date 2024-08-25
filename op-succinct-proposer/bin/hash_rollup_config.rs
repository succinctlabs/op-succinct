use cargo_metadata::MetadataCommand;
use client_utils::boot::hash_rollup_config;
use std::{fs::File, io::Read};

fn main() {
    let metadata = MetadataCommand::new().exec().unwrap();
    let workspace_root = metadata.workspace_root;
    let rollup_config_path = format!("{}/rollup-config.json", workspace_root);

    let mut rollup_config_file = File::open(rollup_config_path).unwrap();
    let mut rollup_config_bytes = Vec::new();
    rollup_config_file
        .read_to_end(&mut rollup_config_bytes)
        .unwrap();

    println!("{:?}", hash_rollup_config(&rollup_config_bytes));
}
