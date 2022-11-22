fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src")
        .build_client(true)
        .build_server(true)
        .compile(&["proto/gate.proto"], &["proto"])?;
    Ok(())
}
