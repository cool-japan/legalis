fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only compile protos if the grpc feature is enabled
    #[cfg(feature = "grpc")]
    {
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
        let descriptor_path = out_dir.join("legalis_descriptor.bin");

        tonic_prost_build::configure()
            .build_server(true)
            .build_client(true)
            .file_descriptor_set_path(&descriptor_path)
            .compile_protos(&["proto/legalis.proto"], &["proto"])?;
    }

    Ok(())
}
