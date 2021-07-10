/*
fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("./proto/pokemon.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/pokemon.proto")?;
    Ok(())
}