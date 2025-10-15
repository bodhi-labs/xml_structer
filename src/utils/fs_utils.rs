use anyhow::{Context, Result};
use jwalk::WalkDir;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Recursively find all XML files in a directory
pub fn find_xml_files(dir: &Path, extensions: &[String], max_depth: usize) -> Result<Vec<String>> {
    info!("Scanning directory: {}", dir.display());

    let mut xml_files = Vec::new();

    let walker = if max_depth > 0 {
        WalkDir::new(dir).max_depth(max_depth)
    } else {
        WalkDir::new(dir)
    };

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if extensions.iter().any(|e| e == ext_str) {
                                let path_str = path.to_string_lossy().to_string();
                                debug!("Found XML file: {}", path_str);
                                xml_files.push(path_str);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                // Log error but continue processing
                tracing::warn!("Error accessing path: {}", e);
            }
        }
    }

    info!("Found {} XML files", xml_files.len());

    if xml_files.is_empty() {
        anyhow::bail!("No XML files found in directory: {}", dir.display());
    }

    Ok(xml_files)
}

/// Validate that a path exists and is a directory
pub fn validate_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    if !path.is_dir() {
        anyhow::bail!("Path is not a directory: {}", path.display());
    }

    Ok(())
}

/// Get the canonical path for a given path
#[allow(unused)]
pub fn get_canonical_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_xml_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create test XML files
        fs::write(temp_path.join("test1.xml"), "<root/>").unwrap();
        fs::write(temp_path.join("test2.xml"), "<root/>").unwrap();
        fs::write(temp_path.join("test.txt"), "not xml").unwrap();

        let extensions = vec!["xml".to_string()];
        let files = find_xml_files(temp_path, &extensions, 0).unwrap();

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_validate_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_directory(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_nonexistent_directory() {
        let path = Path::new("/this/does/not/exist");
        let result = validate_directory(path);
        assert!(result.is_err());
    }
}
