use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// Represents the structural skeleton of an XML element
/// Includes element name, attribute keys (not values), and child structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct XmlStructure {
    /// Element name (e.g., "book", "TEI", "title")
    pub name: String,
    
    /// Attribute keys only (values ignored for structural comparison)
    /// Using BTreeMap for deterministic ordering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, ()>>,
    
    /// Child elements (recursively defined)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<Box<XmlStructure>>,
}

impl XmlStructure {
    /// Create a new XML structure node
    pub fn new(name: String) -> Self {
        Self {
            name,
            attributes: None,
            children: Vec::new(),
        }
    }

    /// Add an attribute key (value is ignored)
    pub fn add_attribute(&mut self, key: String) {
        self.attributes
            .get_or_insert_with(BTreeMap::new)
            .insert(key, ());
    }

    /// Add a child element
    pub fn add_child(&mut self, child: XmlStructure) {
        self.children.push(Box::new(child));
    }

    /// Generate a compact signature string for this structure
    /// Format: name[attr1,attr2]{child1,child2}
    pub fn signature(&self) -> String {
        let mut sig = self.name.clone();
        
        if let Some(attrs) = &self.attributes {
            if !attrs.is_empty() {
                let attr_keys: Vec<&String> = attrs.keys().collect();
                sig.push('[');
                sig.push_str(&attr_keys.iter()
                    .map(|k| k.as_str())
                    .collect::<Vec<_>>()
                    .join(","));
                sig.push(']');
            }
        }
        
        if !self.children.is_empty() {
            sig.push('{');
            sig.push_str(&self.children.iter()
                .map(|c| c.signature())
                .collect::<Vec<_>>()
                .join(","));
            sig.push('}');
        }
        
        sig
    }

    /// Generate a hash for this structure for grouping
    pub fn structure_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for XmlStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        
        if let Some(attrs) = &self.attributes {
            // Hash attribute keys in sorted order
            for key in attrs.keys() {
                key.hash(state);
            }
        }
        
        // Hash children
        for child in &self.children {
            child.hash(state);
        }
    }
}

/// Groups files by their structural signature
#[derive(Debug, Serialize, Deserialize)]
pub struct StructureGroup {
    /// Unique signature for this structure
    pub signature: String,
    
    /// Hash of the structure (for quick comparison)
    pub hash: u64,
    
    /// The actual XML structure
    pub structure: XmlStructure,
    
    /// List of file paths with this structure
    pub files: Vec<String>,
    
    /// Number of files in this group
    pub count: usize,
}

impl StructureGroup {
    pub fn new(structure: XmlStructure, file_path: String) -> Self {
        let signature = structure.signature();
        let hash = structure.structure_hash();
        
        Self {
            signature,
            hash,
            structure,
            files: vec![file_path],
            count: 1,
        }
    }

    pub fn add_file(&mut self, file_path: String) {
        self.files.push(file_path);
        self.count += 1;
    }
}

/// Result of processing all XML files
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResult {
    /// Total number of files processed
    pub total_files: usize,
    
    /// Number of unique structures found
    pub unique_structures: usize,
    
    /// All structure groups
    pub groups: Vec<StructureGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_structure_basic() {
        let mut root = XmlStructure::new("book".to_string());
        root.add_attribute("id".to_string());
        
        let mut title = XmlStructure::new("title".to_string());
        title.add_attribute("lang".to_string());
        
        root.add_child(title);
        
        assert_eq!(root.name, "book");
        assert!(root.attributes.is_some());
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_signature_generation() {
        let mut root = XmlStructure::new("book".to_string());
        root.add_attribute("id".to_string());
        root.add_attribute("type".to_string());
        
        let title = XmlStructure::new("title".to_string());
        root.add_child(title);
        
        let sig = root.signature();
        assert!(sig.contains("book"));
        assert!(sig.contains("id"));
        assert!(sig.contains("type"));
        assert!(sig.contains("title"));
    }

    #[test]
    fn test_structure_equality() {
        let mut s1 = XmlStructure::new("book".to_string());
        s1.add_attribute("id".to_string());
        
        let mut s2 = XmlStructure::new("book".to_string());
        s2.add_attribute("id".to_string());
        
        assert_eq!(s1, s2);
        assert_eq!(s1.structure_hash(), s2.structure_hash());
    }
}
