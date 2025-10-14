use crate::processor::{ProcessingResult, StructureGroup, XmlStructure};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use roxmltree::Document;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info};

/// Process a single XML file and extract its structure
pub fn parse_xml_structure(xml_content: &str) -> Result<XmlStructure> {
    let doc = Document::parse(xml_content).context("Failed to parse XML document")?;

    let root = doc.root_element();
    Ok(build_structure_from_node(&root))
}

/// Recursively build XmlStructure from roxmltree Node
fn build_structure_from_node(node: &roxmltree::Node) -> XmlStructure {
    let mut structure = XmlStructure::new(node.tag_name().name().to_string());

    // Add attribute keys (ignore values)
    for attr in node.attributes() {
        structure.add_attribute(attr.name().to_string());
    }

    // Process child elements (skip text nodes, comments, etc.)
    for child in node.children() {
        if child.is_element() {
            let child_structure = build_structure_from_node(&child);
            structure.add_child(child_structure);
        }
    }

    structure
}

/// Process multiple XML files in parallel
pub fn process_xml_files(
    file_paths: Vec<String>,
    progress_bar: Option<ProgressBar>,
) -> Result<ProcessingResult> {
    info!("Starting to process {} XML files", file_paths.len());

    // Thread-safe map to group files by structure
    let groups_map: Arc<Mutex<HashMap<u64, StructureGroup>>> = Arc::new(Mutex::new(HashMap::new()));

    // Process files in parallel
    file_paths.par_iter().for_each(|file_path| {
        match process_single_file(file_path, &groups_map) {
            Ok(_) => {
                debug!("Successfully processed: {}", file_path);
            }
            Err(e) => {
                error!("Failed to process {}: {}", file_path, e);
            }
        }

        if let Some(ref pb) = progress_bar {
            pb.inc(1);
        }
    });

    if let Some(ref pb) = progress_bar {
        pb.finish_with_message("Processing complete");
    }

    // Convert HashMap to Vec of groups
    let groups_map = Arc::try_unwrap(groups_map)
        .map_err(|_| anyhow::anyhow!("Failed to unwrap Arc"))?
        .into_inner()?;

    let mut groups: Vec<StructureGroup> = groups_map.into_values().collect();

    // Sort by count (descending) for better readability
    groups.sort_by(|a, b| b.count.cmp(&a.count));

    let result = ProcessingResult {
        total_files: file_paths.len(),
        unique_structures: groups.len(),
        groups,
    };

    info!(
        "Processing complete: {} files, {} unique structures",
        result.total_files, result.unique_structures
    );

    Ok(result)
}

/// Process a single XML file and add to groups map
fn process_single_file(
    file_path: &str,
    groups_map: &Arc<Mutex<HashMap<u64, StructureGroup>>>,
) -> Result<()> {
    // Read file
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    // Parse structure
    let structure = parse_xml_structure(&content)
        .with_context(|| format!("Failed to parse XML structure: {}", file_path))?;

    let hash = structure.structure_hash();

    // Add to groups map
    let mut groups = groups_map.lock().unwrap();

    groups
        .entry(hash)
        .and_modify(|group| group.add_file(file_path.to_string()))
        .or_insert_with(|| StructureGroup::new(structure, file_path.to_string()));

    Ok(())
}

/// Create a progress bar for file processing
pub fn create_progress_bar(total: usize) -> ProgressBar {
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}"
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

/// Write processing result to JSON file
pub fn write_result_to_file(
    result: &ProcessingResult,
    output_path: &Path,
    pretty: bool,
) -> Result<()> {
    info!("Writing results to: {}", output_path.display());

    let json = if pretty {
        serde_json::to_string_pretty(result)?
    } else {
        serde_json::to_string(result)?
    };

    fs::write(output_path, json)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    info!("Successfully wrote results to {}", output_path.display());
    Ok(())
}

/// Print summary statistics
pub fn print_summary(result: &ProcessingResult) {
    println!("\n📊 Processing Summary:");
    println!("  Total files processed: {}", result.total_files);
    println!("  Unique structures found: {}", result.unique_structures);
    println!("\n🔍 Top 5 most common structures:");

    for (i, group) in result.groups.iter().take(5).enumerate() {
        println!(
            "  {}. {} files with structure: {}",
            i + 1,
            group.count,
            if group.signature.len() > 80 {
                format!("{}...", &group.signature[..80])
            } else {
                group.signature.clone()
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_xml() {
        let xml = r#"<book id="123"><title>Test</title></book>"#;
        let result = parse_xml_structure(xml);

        assert!(result.is_ok());
        let structure = result.unwrap();
        assert_eq!(structure.name, "book");
        assert!(structure.attributes.is_some());
        assert_eq!(structure.children.len(), 1);
    }

    #[test]
    fn test_parse_nested_xml() {
        let xml = r#"
        <book>
            <metadata>
                <author>John Doe</author>
                <year>2024</year>
            </metadata>
            <content>
                <chapter id="1">Chapter 1</chapter>
                <chapter id="2">Chapter 2</chapter>
            </content>
        </book>
        "#;

        let result = parse_xml_structure(xml);
        assert!(result.is_ok());

        let structure = result.unwrap();
        assert_eq!(structure.name, "book");
        assert_eq!(structure.children.len(), 2);
    }

    #[test]
    fn test_attribute_keys_only() {
        let xml = r#"<book id="123" type="fiction" lang="en"></book>"#;
        let structure = parse_xml_structure(xml).unwrap();

        let attrs = structure.attributes.unwrap();
        assert_eq!(attrs.len(), 3);
        assert!(attrs.contains_key("id"));
        assert!(attrs.contains_key("type"));
        assert!(attrs.contains_key("lang"));
    }
}
