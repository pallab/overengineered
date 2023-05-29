
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src")
        .compile(
            &["../proto/words.proto"], &["../proto"],
        ).unwrap();

   // tonic_build::compile_protos("../proto/grpc.proto")?;
    Ok(())
}