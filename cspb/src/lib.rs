pub mod codec;
mod r#enum {
    include!("./cspb.r#enum.rs");
}
mod msg {
    include!("./cspb.msg.rs");
}

mod registry {
    include!("./cspb.registry.rs");
}

pub use msg::*;
pub use prost::Message;
pub use r#enum::{ErrCode, Gender};
pub use registry::{cs_proto::Payload as CsMsg, sc_proto::Payload as ScMsg, CsProto, ScProto};

#[cfg(test)]
mod test {
    use super::CsMsg;
    use serde::Deserialize;
    use serde::Serialize;
    use std::string::ToString;
    use strum::EnumIter;
    use strum::EnumVariantNames;
    use strum::IntoEnumIterator;
    use strum::VariantNames;
    #[test]
    fn foo() {
        let msg = CsMsg::CsEcho(crate::CsEcho {
            content: "Hello".to_string(),
        });
        let ret = serde_json::to_string(&msg).unwrap();
        println!("{}", ret);
    }

    #[derive(EnumIter, Serialize, Deserialize, Debug, EnumVariantNames)]
    enum MyEnum {
        CsLogin(super::CsLogin),
        CsFastLogin(super::CsFastLogin),
    }
    #[test]
    fn bar() {
        MyEnum::VARIANTS.iter().for_each(|name| {
            println!("{}", name);
        });
        MyEnum::iter().for_each(|f| {
            let ret = serde_json::to_string_pretty(&f).unwrap();
            println!("{}", ret);
        })
    }
}
