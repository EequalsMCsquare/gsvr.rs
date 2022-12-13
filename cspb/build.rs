use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .out_dir("src/")
        .type_attribute(
            ".cspb.msg",
            "#[derive(::serde::Serialize, ::serde::Deserialize, ::gsfw::Protocol)]",
        )
        .type_attribute(
            ".cspb.Gender",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(
            ".cspb.msg",
            "#[protocol(registry=\"super::registry::MsgId\")]",
        )
        .type_attribute(
            "cspb.Enum.Gender",
            "#[derive(::serde::Serialize, ::serde::Deserialize)]",
        )
        .type_attribute(".cspb.registry.MsgId", "#[derive(::gsfw::Registry)]")
        .type_attribute(
            ".cspb.registry.MsgId",
            "#[registry(prefix=\"super::msg::\",rename=\"Registry\")]",
        )
        .field_attribute(".cspb.registry.MsgId.__RESERVED", "#[registry(skip)]")
        .compile_protos(&["proto/registry.proto"], &["proto"])?;
    Ok(())
}
