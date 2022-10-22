use std::{io, path::Path};

pub use tracing::Level;
use tracing_subscriber::EnvFilter;

use gconf::ConfigLog;

/// initialize logger for an app
/// # example
/// ```
/// // console will display "foo"
/// init_logger("stdout");
/// tracing::debug!("foo");
///
/// // normal log file
/// init_logger("file:log");
/// tracing::debug!("bar");
///
/// // a daily rotate log file
/// init_logger("rfile:log");
/// tracing::debug!("foo bar");
///
/// // fallback to stdout
/// init_logger("klsdfjg");
/// tracing::debug!("console will display this message");
/// ```
pub fn init_logger(cfg: ConfigLog) {
    let builder = tracing_subscriber::FmtSubscriber::builder()
        .with_level(true)
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stdout)
        .with_file(true)
        .with_line_number(true);
    let output = cfg.output.unwrap_or("stdout".to_string());
    match output.as_str() {
        "stdout" => builder.with_writer(io::stdout).init(),
        "stderr" => builder.with_writer(io::stderr).init(),
        &_ => {
            if output.starts_with("file:") {
                let output_path = Path::new(&output.replace("file:", "")).to_path_buf();
                let dir = output_path.parent();
                let file_name = output_path.file_name().unwrap();
                let file_appender =
                    tracing_appender::rolling::never(dir.unwrap_or(Path::new("./")), file_name);
                let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
                builder.with_writer(non_blocking_writer).init();
            } else if output.starts_with("rfile:") {
                let output_path = Path::new(&output.replace("rfile:", "")).to_path_buf();
                let dir = output_path.parent();
                let file_name = output_path.file_name().unwrap();
                let file_appender =
                    tracing_appender::rolling::daily(dir.unwrap_or(Path::new("./")), file_name);
                let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
                builder.with_writer(non_blocking_writer).init();
            } else {
                builder.with_writer(io::stdout).init();
                tracing::warn!("invalid log output: {}, failback to stdout", output);
            }
        }
    };
    tracing::debug!("logger init success");
}
