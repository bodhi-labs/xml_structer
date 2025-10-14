use clap::Parser;
use std::path::PathBuf;

/// XML Structure Analyzer - Parse and group TEI XML files by their structural skeleton
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory containing XML files to process
    #[arg(value_name = "DIRECTORY")]
    pub input_dir: PathBuf,

    /// Output JSON file path
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE", default_value = "config/default.toml")]
    pub config: String,

    /// Number of parallel threads (0 = auto-detect)
    #[arg(short = 't', long)]
    pub threads: Option<usize>,

    /// Maximum directory traversal depth (0 = unlimited)
    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short = 'l', long, default_value = "info")]
    pub log_level: String,

    /// Disable progress bar
    #[arg(long)]
    pub no_progress: bool,

    /// Disable pretty-print JSON output
    #[arg(long)]
    pub no_pretty: bool,

    /// Verbose output (equivalent to --log-level debug)
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    /// Get the effective log level
    pub fn effective_log_level(&self) -> String {
        if self.verbose {
            "debug".to_string()
        } else {
            self.log_level.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbose_overrides_log_level() {
        let cli = Cli {
            input_dir: PathBuf::from("."),
            output: None,
            config: "config/default.toml".to_string(),
            threads: None,
            max_depth: None,
            log_level: "info".to_string(),
            no_progress: false,
            no_pretty: false,
            verbose: true,
        };
        
        assert_eq!(cli.effective_log_level(), "debug");
    }

    #[test]
    fn test_default_log_level() {
        let cli = Cli {
            input_dir: PathBuf::from("."),
            output: None,
            config: "config/default.toml".to_string(),
            threads: None,
            max_depth: None,
            log_level: "info".to_string(),
            no_progress: false,
            no_pretty: false,
            verbose: false,
        };
        
        assert_eq!(cli.effective_log_level(), "info");
    }
}
