pub mod struct_processor;
pub mod xml_struct;

pub use struct_processor::{
    create_progress_bar, parse_xml_structure, print_summary, process_xml_files,
    write_result_to_file,
};

pub use xml_struct::{ProcessingResult, StructureGroup, XmlStructure};
