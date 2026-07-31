use op_succinct_build_utils::build_all;

fn main() {
    build_all();

    #[cfg(feature = "agglayer")]
    {
        println!("cargo:rerun-if-changed=proto/proofs.proto");
        tonic_build::compile_protos("proto/proofs.proto")
            .expect("failed to compile proto/proofs.proto");
    }
}
