use crate::gconf::ConfigLog;
use std::path::Path;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{EnvFilter, fmt::time::UtcTime};

pub fn init(cfg: ConfigLog) -> WorkerGuard {
    let builder = tracing_subscriber::FmtSubscriber::builder()
        .json()
        .with_timer(UtcTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(cfg.enable_level.unwrap_or(true))
        .with_file(cfg.enable_file.unwrap_or(true))
        .with_line_number(cfg.enable_line.unwrap_or(true))
        .with_thread_ids(cfg.enable_thread_id.unwrap_or(false))
        .with_thread_names(cfg.enable_thread_name.unwrap_or(false));
    let (writer, guard) = parse_output(cfg.output.unwrap_or("stdout".into()));
    let subscriber = builder.with_writer(writer).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    guard
}

fn parse_output(output: String) -> (NonBlocking, WorkerGuard) {
    match output.as_str() {
        "stdout" => tracing_appender::non_blocking(std::io::stdout()),
        "stderr" => tracing_appender::non_blocking(std::io::stderr()),
        _path => {
            if output.starts_with("file:") {
                let output_path = Path::new(&_path.replace("file:", "")).to_path_buf();
                let dir = output_path.parent();
                let file_name = output_path.file_name().unwrap();
                let file_appender =
                    tracing_appender::rolling::never(dir.unwrap_or(Path::new("./")), file_name);
                tracing_appender::non_blocking(file_appender)
            } else if output.starts_with("rfile:") {
                let output_path = Path::new(&output.replace("rfile:", "")).to_path_buf();
                let dir = output_path.parent();
                let file_name = output_path.file_name().unwrap();
                let file_appender =
                    tracing_appender::rolling::daily(dir.unwrap_or(Path::new("./")), file_name);
                tracing_appender::non_blocking(file_appender)
            } else {
                tracing::warn!("invalid log output: {}, failback to stdout", output);
                tracing_appender::non_blocking(std::io::stdout())
            }
        }
    }
}
