pub mod cli;
pub mod processor;
pub mod utils;
pub mod validation;
pub mod wasm;
pub mod xsconfig;

pub use cli::Cli;
pub use processor::{struct_processor, xml_struct};
pub use validation::{report, validate};
pub use xsconfig::{LoggingConfig, OutputConfig, ProcessingConfig, XsConfig};

/// One-call entry point.
pub fn validate(path: impl AsRef<std::path::Path>) -> anyhow::Result<report::Report> {
    let xml = std::fs::read_to_string(path)?;
    validate::run(&xml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broken_xml() {
        let report = validate::run("<not>closed").unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.errors.len(), 1);
        // Just verify we got an error, don't check exact text
        assert!(!report.errors[0].text.is_empty());
    }

    #[test]
    fn test_valid_tei() {
        let xml = r#"<TEI><text><body><div><head>Title</head></div></body></text></TEI>"#;
        let report = validate::run(xml).unwrap();
        assert!(report.is_valid());
        assert_eq!(report.errors.len(), 0);
    }

    #[test]
    fn test_pb_missing_attributes() {
        let xml = r#"<TEI><text><body><pb/></body></text></TEI>"#;
        let report = validate::run(xml).unwrap();
        assert!(!report.is_valid());
        assert_eq!(report.errors.len(), 2); // missing @ed and @n
    }

    #[test]
    fn test_head_outside_div() {
        let xml = r#"<TEI><text><body><head>Title</head></body></text></TEI>"#;
        let report = validate::run(xml).unwrap();
        assert!(report.is_valid()); // just a warning
        assert_eq!(report.warnings.len(), 1);
        assert!(report.warnings[0]
            .text
            .contains("<head> should be inside <div>"));
    }
}
