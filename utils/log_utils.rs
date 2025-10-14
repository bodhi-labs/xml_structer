use anyhow::Result;
use std::path::Path;
use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging with optional file output
pub fn init_logging(level: &str, log_file: Option<&Path>) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))?;
    
    let fmt_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false);
    
    if let Some(log_path) = log_file {
        let file = std::fs::File::create(log_path)?;
        let file_layer = fmt::layer()
            .with_writer(file)
            .with_ansi(false);
        
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(file_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
    
    Ok(())
}

/// Parse log level string to tracing::Level
pub fn parse_log_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            eprintln!("Invalid log level '{}', defaulting to INFO", level);
            Level::INFO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_level() {
        assert_eq!(parse_log_level("trace"), Level::TRACE);
        assert_eq!(parse_log_level("DEBUG"), Level::DEBUG);
        assert_eq!(parse_log_level("info"), Level::INFO);
        assert_eq!(parse_log_level("WARN"), Level::WARN);
        assert_eq!(parse_log_level("error"), Level::ERROR);
        assert_eq!(parse_log_level("invalid"), Level::INFO);
    }
}
