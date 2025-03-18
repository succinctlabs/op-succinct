use op_succinct_build_utils::build_all;

fn main() {
    build_all();

    tonic_build::configure()
        .build_server(true)
        .compile_protos(&["proto/agglayer.proto"], &["proto"])
        .unwrap();
}
