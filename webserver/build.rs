
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src")  // you can change the generated code's location
        .compile(
            &["../proto/stocks.proto"], &["../proto"],
        ).unwrap();

   // tonic_build::compile_protos("../proto/grpc.proto")?;
    Ok(())
}