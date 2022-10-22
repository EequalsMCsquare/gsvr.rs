use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/")
        .type_attribute(
            ".pb.msg",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".pb.registry.CsProto",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".pb.registry.CsProto.payload",
            "#[derive(::strum::EnumIter, ::strum::EnumVariantNames)]",
        )
        .type_attribute(
            ".pb.registry.ScProto",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".pb.registry.ScProto.payload",
            "#[derive(::strum::EnumIter, ::strum::EnumVariantNames)]",
        )
        .compile_protos(&["proto/registry.proto"], &["proto"])?;
    Ok(())
}
