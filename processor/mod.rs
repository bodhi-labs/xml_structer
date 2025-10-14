pub mod xml_processor;

pub use xml_processor::{
    create_progress_bar, parse_xml_structure, print_summary, process_xml_files,
    write_result_to_file,
};
