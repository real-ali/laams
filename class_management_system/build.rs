fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./proto/class/processor_service.proto")?;

    // tonic_build::configure()
    //     .out_dir("src/presentation/grpc/gen/")
    //     .proto_path("proto/")
    //     .build_server(true)
    //     .build_client(true)
    //     .compile(
    //         &["./proto/class/processor_service.proto"],
    //         &["./proto/class"],
    //     )?;
    Ok(())
}
