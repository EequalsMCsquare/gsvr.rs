use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/")
        .type_attribute(
            ".cspb.msg",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            "cspb.Enum.Gender",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".cspb.registry.CsProto",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".cspb.registry.CsProto.payload",
            "#[derive(::strum::EnumIter, ::strum::EnumVariantNames)]",
        )
        .type_attribute(
            ".cspb.registry.ScProto",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".cspb.registry.ScProto.payload",
            "#[derive(::strum::EnumIter, ::strum::EnumVariantNames)]",
        )
        .compile_protos(&["proto/registry.proto"], &["proto"])?;
    Ok(())
}
