use op_succinct_build_utils::build_all;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_all();

    tonic_build::compile_protos("proto/proofs.proto")?;
    Ok(())
}
