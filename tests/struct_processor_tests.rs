use std::fs;
use tempfile::TempDir;

use xml_structer::processor::{parse_xml_structure, process_xml_files};

#[test]
fn test_parse_simple_book() {
    let xml = r#"
    <book>
        <title>Test Book</title>
        <author>Test Author</author>
        <year>2024</year>
    </book>
    "#;

    let structure = parse_xml_structure(xml).unwrap();

    assert_eq!(structure.name, "book");
    assert_eq!(structure.children.len(), 3);

    let child_names: Vec<String> = structure.children.iter().map(|c| c.name.clone()).collect();

    assert!(child_names.contains(&"title".to_string()));
    assert!(child_names.contains(&"author".to_string()));
    assert!(child_names.contains(&"year".to_string()));
}

#[test]
fn test_parse_with_attributes() {
    let xml = r#"<book id="123" type="fiction"><title lang="en">Test</title></book>"#;

    let structure = parse_xml_structure(xml).unwrap();

    assert_eq!(structure.name, "book");
    assert!(structure.attributes.is_some());

    let attrs = structure.attributes.as_ref().unwrap();
    assert!(attrs.contains_key("id"));
    assert!(attrs.contains_key("type"));

    assert_eq!(structure.children.len(), 1);
    let title = &structure.children[0];
    assert_eq!(title.name, "title");
    assert!(title.attributes.as_ref().unwrap().contains_key("lang"));
}

#[test]
fn test_same_structure_different_values() {
    let xml1 = r#"<book id="123"><title>Book One</title></book>"#;
    let xml2 = r#"<book id="456"><title>Book Two</title></book>"#;

    let structure1 = parse_xml_structure(xml1).unwrap();
    let structure2 = parse_xml_structure(xml2).unwrap();

    // Should have same structure despite different values
    assert_eq!(structure1, structure2);
    assert_eq!(structure1.structure_hash(), structure2.structure_hash());
}

#[test]
fn test_different_structures() {
    let xml1 = r#"<book><title>Test</title></book>"#;
    let xml2 = r#"<book><author>Test</author></book>"#;

    let structure1 = parse_xml_structure(xml1).unwrap();
    let structure2 = parse_xml_structure(xml2).unwrap();

    assert_ne!(structure1, structure2);
    assert_ne!(structure1.structure_hash(), structure2.structure_hash());
}

#[test]
fn test_structure_signature() {
    let xml = r#"<book id="1"><title>Test</title><author>Name</author></book>"#;

    let structure = parse_xml_structure(xml).unwrap();
    let signature = structure.signature();

    assert!(signature.contains("book"));
    assert!(signature.contains("id"));
    assert!(signature.contains("title"));
    assert!(signature.contains("author"));
}

#[test]
fn test_deeply_nested_structure() {
    let xml = r#"
    <root>
        <level1>
            <level2>
                <level3>
                    <level4>Deep content</level4>
                </level3>
            </level2>
        </level1>
    </root>
    "#;

    let structure = parse_xml_structure(xml).unwrap();

    assert_eq!(structure.name, "root");
    assert_eq!(structure.children.len(), 1);

    let level1 = &structure.children[0];
    assert_eq!(level1.name, "level1");
    assert_eq!(level1.children.len(), 1);

    let level2 = &level1.children[0];
    assert_eq!(level2.name, "level2");
}

#[test]
fn test_process_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create test files with same structure
    let xml1 = r#"<book><title>Book 1</title><author>Author 1</author></book>"#;
    let xml2 = r#"<book><title>Book 2</title><author>Author 2</author></book>"#;

    let file1 = temp_path.join("book1.xml");
    let file2 = temp_path.join("book2.xml");

    fs::write(&file1, xml1).unwrap();
    fs::write(&file2, xml2).unwrap();

    let files = vec![
        file1.to_string_lossy().to_string(),
        file2.to_string_lossy().to_string(),
    ];

    let result = process_xml_files(files, None).unwrap();

    assert_eq!(result.total_files, 2);
    assert_eq!(result.unique_structures, 1); // Same structure
    assert_eq!(result.groups[0].count, 2);
}

#[test]
fn test_process_different_structures() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    let xml1 = r#"<book><title>Test</title></book>"#;
    let xml2 = r#"<article><heading>Test</heading></article>"#;

    let file1 = temp_path.join("doc1.xml");
    let file2 = temp_path.join("doc2.xml");

    fs::write(&file1, xml1).unwrap();
    fs::write(&file2, xml2).unwrap();

    let files = vec![
        file1.to_string_lossy().to_string(),
        file2.to_string_lossy().to_string(),
    ];

    let result = process_xml_files(files, None).unwrap();

    assert_eq!(result.total_files, 2);
    assert_eq!(result.unique_structures, 2); // Different structures
}

#[test]
fn test_attribute_order_doesnt_matter() {
    let xml1 = r#"<book id="1" type="fiction" lang="en"></book>"#;
    let xml2 = r#"<book lang="en" type="fiction" id="2"></book>"#;

    let structure1 = parse_xml_structure(xml1).unwrap();
    let structure2 = parse_xml_structure(xml2).unwrap();

    // Should be equal despite different attribute order (BTreeMap ensures sorted order)
    assert_eq!(structure1, structure2);
}

#[test]
fn test_empty_element() {
    let xml = r#"<book><empty/></book>"#;

    let structure = parse_xml_structure(xml).unwrap();

    assert_eq!(structure.name, "book");
    assert_eq!(structure.children.len(), 1);
    assert_eq!(structure.children[0].name, "empty");
    assert!(structure.children[0].children.is_empty());
}
