use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// Represents the full structural tree of an XML element
/// This is the complete parsed structure (can be large)
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

/// Compact skeleton signature - merges duplicate child elements
/// This is what gets compared and stored (much smaller)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkeletonSignature {
    /// Root element name
    pub root: String,

    /// Compact JSON representation of the structure
    /// Merges duplicate children and only keeps unique structure
    pub skeleton: Value,

    /// Hash of the skeleton for quick comparison
    pub hash: u64,
}

impl SkeletonSignature {
    /// Generate a skeleton signature from a full XML structure
    /// This merges duplicate child elements into a compact form
    pub fn from_structure(structure: &XmlStructure) -> Self {
        let skeleton = Self::build_skeleton_json(structure);
        let hash = Self::hash_skeleton(&skeleton);

        Self {
            root: structure.name.clone(),
            skeleton,
            hash,
        }
    }

    /// Build compact JSON skeleton (like tree_summary in your code)
    fn build_skeleton_json(node: &XmlStructure) -> Value {
        let mut summary_map = Map::new();

        // Add attributes if present
        if let Some(attrs) = &node.attributes {
            let mut attr_list: Vec<String> = attrs.keys().cloned().collect();
            attr_list.sort();
            if !attr_list.is_empty() {
                summary_map.insert("@attributes".to_string(), json!(attr_list));
            }
        }

        // Group children by name and merge their structures
        let mut children_by_name: BTreeMap<String, Vec<&XmlStructure>> = BTreeMap::new();
        for child in &node.children {
            children_by_name
                .entry(child.name.clone())
                .or_insert_with(Vec::new)
                .push(child);
        }

        // For each unique child name, merge all instances
        for (child_name, instances) in children_by_name {
            let merged_child = Self::merge_child_instances(instances);
            summary_map.insert(child_name, merged_child);
        }

        summary_map.into()
    }

    /// Merge multiple instances of the same child element
    fn merge_child_instances(instances: Vec<&XmlStructure>) -> Value {
        if instances.is_empty() {
            return json!({});
        }

        // Start with first instance
        let mut merged = Self::build_skeleton_json(instances[0]);

        // Merge in remaining instances
        for instance in instances.iter().skip(1) {
            let instance_json = Self::build_skeleton_json(instance);
            Self::merge_skeleton_values(&mut merged, &instance_json);
        }

        merged
    }

    /// Merge two skeleton JSON values (similar to your merge_values)
    fn merge_skeleton_values(existing: &mut Value, new: &Value) {
        if let (Some(existing_map), Some(new_map)) = (existing.as_object_mut(), new.as_object()) {
            for (key, value) in new_map {
                if key == "@attributes" {
                    // Merge attribute lists
                    if let (Some(existing_attrs), Some(new_attrs)) = (
                        existing_map.get_mut(key).and_then(|v| v.as_array_mut()),
                        value.as_array(),
                    ) {
                        for attr in new_attrs {
                            if !existing_attrs.contains(attr) {
                                existing_attrs.push(attr.clone());
                            }
                        }
                        // Keep sorted
                        if let Some(attrs) =
                            existing_map.get_mut(key).and_then(|v| v.as_array_mut())
                        {
                            attrs.sort_by(|a, b| {
                                a.as_str().unwrap_or("").cmp(b.as_str().unwrap_or(""))
                            });
                        }
                    }
                } else {
                    // Merge child elements
                    existing_map
                        .entry(key.clone())
                        .and_modify(|e| Self::merge_skeleton_values(e, value))
                        .or_insert(value.clone());
                }
            }
        }
    }

    /// Generate hash from skeleton JSON for comparison
    fn hash_skeleton(skeleton: &Value) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        // Use canonical JSON string for consistent hashing
        skeleton.to_string().hash(&mut hasher);
        hasher.finish()
    }

    /// Generate a compact string representation of the skeleton
    pub fn to_compact_string(&self) -> String {
        format!("{}:{}", self.root, self.skeleton.to_string())
    }
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

    /// Convert this structure to a compact skeleton signature
    pub fn to_skeleton(&self) -> SkeletonSignature {
        SkeletonSignature::from_structure(self)
    }

    /// Generate a compact signature string for this structure
    /// Format: name[attr1,attr2]{child1,child2}
    #[allow(unused)]
    pub fn signature(&self) -> String {
        let mut sig = self.name.clone();

        if let Some(attrs) = &self.attributes {
            if !attrs.is_empty() {
                let attr_keys: Vec<&String> = attrs.keys().collect();
                sig.push('[');
                sig.push_str(
                    &attr_keys
                        .iter()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(","),
                );
                sig.push(']');
            }
        }

        if !self.children.is_empty() {
            sig.push('{');
            sig.push_str(
                &self
                    .children
                    .iter()
                    .map(|c| c.signature())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            sig.push('}');
        }

        sig
    }

    /// Generate a hash for this structure for grouping
    #[allow(unused)]
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

/// Groups files by their skeleton signature
#[derive(Debug, Serialize, Deserialize)]
pub struct StructureGroup {
    /// Compact skeleton signature (merged, deduplicated structure)
    pub skeleton: SkeletonSignature,

    /// List of file paths with this skeleton
    pub files: Vec<String>,

    /// Number of files in this group
    pub count: usize,

    /// Optional: Store ONE example of the full structure (not all 177!)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_structure: Option<XmlStructure>,
}

impl StructureGroup {
    pub fn new(structure: XmlStructure, file_path: String) -> Self {
        let skeleton = structure.to_skeleton();

        Self {
            skeleton,
            files: vec![file_path],
            count: 1,
            example_structure: Some(structure), // Keep first example
        }
    }

    pub fn add_file(&mut self, file_path: String) {
        self.files.push(file_path);
        self.count += 1;
        // Don't add more structures - we already have an example
    }

    /// Get the hash for comparison
    #[allow(unused)]
    pub fn hash(&self) -> u64 {
        self.skeleton.hash
    }

    /// Get compact string representation
    pub fn signature_string(&self) -> String {
        self.skeleton.to_compact_string()
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
    fn test_skeleton_signature_generation() {
        let mut root = XmlStructure::new("book".to_string());
        root.add_attribute("id".to_string());
        root.add_attribute("type".to_string());

        let title = XmlStructure::new("title".to_string());
        root.add_child(title);

        let skeleton = root.to_skeleton();
        assert_eq!(skeleton.root, "book");

        let skeleton_obj = skeleton.skeleton.as_object().unwrap();
        assert!(skeleton_obj.contains_key("@attributes"));
        assert!(skeleton_obj.contains_key("title"));
    }

    #[test]
    fn test_skeleton_merges_duplicate_children() {
        let mut root = XmlStructure::new("book".to_string());

        // Add two chapter children with same name
        let chapter1 = XmlStructure::new("chapter".to_string());
        let chapter2 = XmlStructure::new("chapter".to_string());

        root.add_child(chapter1);
        root.add_child(chapter2);

        let skeleton = root.to_skeleton();
        let skeleton_obj = skeleton.skeleton.as_object().unwrap();

        // Should only have ONE "chapter" key (merged)
        assert_eq!(skeleton_obj.len(), 1);
        assert!(skeleton_obj.contains_key("chapter"));
    }

    #[test]
    fn test_skeleton_equality_same_structure() {
        let mut s1 = XmlStructure::new("book".to_string());
        s1.add_attribute("id".to_string());
        let title1 = XmlStructure::new("title".to_string());
        s1.add_child(title1);

        let mut s2 = XmlStructure::new("book".to_string());
        s2.add_attribute("id".to_string());
        let title2 = XmlStructure::new("title".to_string());
        s2.add_child(title2);

        let skeleton1 = s1.to_skeleton();
        let skeleton2 = s2.to_skeleton();

        assert_eq!(skeleton1.hash, skeleton2.hash);
        assert_eq!(skeleton1.skeleton, skeleton2.skeleton);
    }

    #[test]
    fn test_skeleton_merges_attributes() {
        let mut root = XmlStructure::new("book".to_string());

        let mut child1 = XmlStructure::new("chapter".to_string());
        child1.add_attribute("id".to_string());

        let mut child2 = XmlStructure::new("chapter".to_string());
        child2.add_attribute("title".to_string());

        root.add_child(child1);
        root.add_child(child2);

        let skeleton = root.to_skeleton();
        let skeleton_obj = skeleton.skeleton.as_object().unwrap();
        let chapter_obj = skeleton_obj.get("chapter").unwrap().as_object().unwrap();
        let attrs = chapter_obj.get("@attributes").unwrap().as_array().unwrap();

        // Should merge both attributes
        assert_eq!(attrs.len(), 2);
        assert!(attrs.contains(&json!("id")));
        assert!(attrs.contains(&json!("title")));
    }
}
