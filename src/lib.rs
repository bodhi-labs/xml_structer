pub mod cli;
pub mod processor;
pub mod utils;
pub mod xsconfig;

pub use cli::Cli;
pub use processor::{
    create_progress_bar, parse_xml_structure, print_summary, process_xml_files, struct_processor,
    write_result_to_file, ProcessingResult, StructureGroup, XmlStructure,
};
pub use xsconfig::{LoggingConfig, OutputConfig, ProcessingConfig, XsConfig};
