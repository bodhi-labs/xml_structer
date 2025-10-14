use anyhow::Result;
use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XsConfig {
    pub processing: ProcessingConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
}

impl XsConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self> {
        let settings = ConfigLoader::builder()
            .add_source(File::with_name(path))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    /// Create default configuration
    pub fn default() -> Self {
        Self {
            processing: ProcessingConfig {
                num_threads: 0,
                max_depth: 0,
                file_extensions: vec!["xml".to_string(), "tei".to_string()],
            },
            output: OutputConfig {
                output_file: "xml_structures.json".to_string(),
                pretty_print: true,
                include_paths: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                log_file: None,
            },
        }
    }

    /// Get log file path if specified
    pub fn log_file_path(&self) -> Option<PathBuf> {
        self.logging.log_file.as_ref().map(PathBuf::from)
    }

    /// Get output file path
    pub fn output_file_path(&self) -> PathBuf {
        PathBuf::from(&self.output.output_file)
    }

    /// Merge with CLI overrides
    pub fn merge_with_cli(mut self, output: Option<String>, threads: Option<usize>) -> Self {
        if let Some(output_path) = output {
            self.output.output_file = output_path;
        }

        if let Some(num_threads) = threads {
            self.processing.num_threads = num_threads;
        }

        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub num_threads: usize,
    pub max_depth: usize,
    pub file_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub output_file: String,
    pub pretty_print: bool,
    pub include_paths: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_file: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = XsConfig::default();
        assert_eq!(config.processing.file_extensions, vec!["xml", "tei"]);
        assert_eq!(config.output.pretty_print, true);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_merge_with_cli() {
        let config = XsConfig::default();
        let merged = config.merge_with_cli(Some("custom_output.json".to_string()), Some(8));

        assert_eq!(merged.output.output_file, "custom_output.json");
        assert_eq!(merged.processing.num_threads, 8);
    }

    #[test]
    fn test_output_file_path() {
        let config = XsConfig::default();
        let path = config.output_file_path();
        assert_eq!(path, PathBuf::from("xml_structures.json"));
    }
}
