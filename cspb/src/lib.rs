include!("./cspb.rs");

mod registry {
    include!("./cspb.registry.rs");
}
mod msg {
    include!("./cspb.msg.rs");
}

pub use msg::*;
pub use prost::Message;
pub use registry::{Registry, MsgId};