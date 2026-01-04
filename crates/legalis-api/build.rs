fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only compile protos if the grpc feature is enabled
    #[cfg(feature = "grpc")]
    {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .protoc_arg("--experimental_allow_proto3_optional")
            .file_descriptor_set_path("proto/legalis_descriptor.bin") // For reflection
            .compile_protos(&["proto/legalis.proto"], &["proto"])?;
    }

    Ok(())
}
