use std::sync::Arc;

use op_succinct_client_utils::{
    client::run_opsuccinct_client, precompiles::zkvm_handle_register, InMemoryOracle,
};

fn main() {
    let bytes = include_bytes!("../../input.bin").to_vec();

    let oracle = Arc::new(InMemoryOracle::from_raw_bytes(bytes));

    oracle.verify().unwrap();

    let oracle_clone = oracle.clone();
    kona_proof::block_on(async move {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });

    let test = [0u8; 128];
    println!("test {test:?}");

    let a = Vec::<u8>::new();
    let oracle_clone = oracle.clone();
    kona_proof::block_on(async {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });
    println!("a {a:?}");
    let b = Vec::<u8>::new();
    let oracle_clone = oracle.clone();
    kona_proof::block_on(async move {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });
    println!("b {b:?}");
    let c = Vec::<u8>::new();
    let oracle_clone = oracle.clone();
    kona_proof::block_on(async move {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });
    println!("c {c:?}");
    let d = Vec::<u8>::new();
    let oracle_clone = oracle.clone();
    kona_proof::block_on(async move {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });
    println!("d {d:?}");
    let e = Vec::<u8>::new();
    let oracle_clone = oracle.clone();
    kona_proof::block_on(async move {
        run_opsuccinct_client(oracle_clone, Some(zkvm_handle_register))
            .await
            .expect("failed to run client");
    });
    println!("e {e:?}");

    println!("boom {a:?}");
}
