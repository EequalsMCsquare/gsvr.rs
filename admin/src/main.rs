mod cmd;
mod exec;
mod proto;

#[tokio::main]
async fn main() {
    util::init_logger(gconf::ConfigLog {
        level: Default::default(),
        output: Some("stdout".to_string()),
    });

}
