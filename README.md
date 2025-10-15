# 🧩 XML Structer

A high-performance Rust tool for parsing and analyzing TEI XML files by their structural skeleton. Groups files by their XML element structure and attribute keys (ignoring attribute values), perfect for analyzing large batches of XML documents.

## 🎯 Features

- **Task 1 (Current)**: Group XML files by structural skeleton
  - Compares elements and attribute keys
  - Ignores attribute values and text content
  - Exports results as JSON
- **Parallel Processing**: Utilizes all CPU cores with Rayon
- **Fast Directory Scanning**: Multi-threaded directory traversal with jwalk
- **Progress Tracking**: Real-time progress bars with indicatif
- **Structured Logging**: Comprehensive logging with tracing
- **Configurable**: TOML configuration with CLI overrides

## 📦 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Build from source

```bash
git clone <repository-url>
cd xml_structer
cargo build --release
```

The binary will be available at `target/release/xml_structer`

## 🚀 Usage

### Basic Usage

```bash
# Process all XML files in a directory
xml_structer /path/to/xml/files

# Specify custom output file
xml_structer /path/to/xml/files -o results.json

# Use custom configuration
xml_structer /path/to/xml/files -c config/custom.toml

# Control thread count
xml_structer /path/to/xml/files -t 8

# Verbose logging
xml_structer /path/to/xml/files -v


```

### Command Line Options

```
Usage: xml_structer [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  Directory containing XML files to process

Options:
  -o, --output <FILE>         Output JSON file path
  -c, --config <FILE>         Configuration file path [default: config/default.toml]
  -t, --threads <THREADS>     Number of parallel threads (0 = auto-detect)
  -d, --max-depth <MAX_DEPTH> Maximum directory traversal depth (0 = unlimited)
  -l, --log-level <LEVEL>     Log level (trace, debug, info, warn, error) [default: info]
      --no-progress           Disable progress bar
      --no-pretty             Disable pretty-print JSON output
  -v, --verbose               Verbose output (equivalent to --log-level debug)
  -h, --help                  Print help
  -V, --version               Print version
```

## ⚙️ Configuration

Configuration file (`config/default.toml`):

```toml
[processing]
# Number of parallel threads (0 = auto-detect)
num_threads = 0

# Maximum depth for directory traversal (0 = unlimited)
max_depth = 0

# File extensions to process
file_extensions = ["xml", "tei"]

[output]
# Output file for structural signatures
output_file = "xml_structures.json"

# Pretty print JSON output
pretty_print = true

# Include file paths in output
include_paths = true

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Optional log file path
# log_file = "xml_structer.log"
```

## 📊 Output Format

The tool generates a JSON file with the following structure:

```json
{
  "total_files": 100,
  "unique_structures": 5,
  "groups": [
    {
      "signature": "book[id,type]{title,author{name},year}",
      "hash": 12345678901234567890,
      "structure": {
        "name": "book",
        "attributes": {
          "id": null,
          "type": null
        },
        "children": [
          {
            "name": "title",
            "children": []
          },
          {
            "name": "author",
            "children": [
              {
                "name": "name",
                "children": []
              }
            ]
          },
          {
            "name": "year",
            "children": []
          }
        ]
      },
      "files": [
        "/path/to/file1.xml",
        "/path/to/file2.xml"
      ],
      "count": 2
    }
  ]
}
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_simple_book

# Run tests with verbose logging
RUST_LOG=debug cargo test
```

## 📁 Project Structure

```
xml_structer/
├── Cargo.toml              # Dependencies and project metadata
├── config/
│   └── default.toml        # Default configuration
├── src/
│   ├── main.rs             # Application entry point
│   ├── cli.rs              # Command-line interface
│   ├── config.rs           # Configuration management
│   ├── model/
│   │   ├── mod.rs          # Model module
│   │   └── xml_node.rs     # XML structure data types
│   ├── processor/
│   │   ├── mod.rs          # Processor module
│   │   └── xml_processor.rs # XML parsing and processing logic
│   └── utils/
│       ├── mod.rs          # Utilities module
│       ├── fs_utils.rs     # File system utilities
│       └── log_utils.rs    # Logging utilities
└── tests/
    ├── fixtures/           # Test XML files
    │   ├── book1.xml
    │   ├── book2.xml
    │   ├── book_with_attrs.xml
    │   └── nested_with_attrs.xml
    └── xml_processor_tests.rs # Integration tests
```

## 🔧 Dependencies

| Purpose | Crate | Reason |
|---------|-------|--------|
| XML Parsing | `roxmltree` | Fast, read-only XML DOM tree |
| Parallel Directory Traversal | `jwalk` | Threaded recursive directory walker |
| Parallel Processing | `rayon` | Thread pool for concurrent file parsing |
| Progress Display | `indicatif` | Elegant progress bar output |
| Logging | `tracing` + `tracing-subscriber` | Structured async logging |
| Serialization | `serde` + `serde_json` | JSON serialization |
| CLI | `clap` | Command-line argument parsing |
| Configuration | `config` | Configuration file management |
| Error Handling | `anyhow` + `thiserror` | Comprehensive error handling |

## 🎯 Roadmap

- [x] **Task 1**: Group XML files by structural skeleton
- [ ] **Task 2**: Search functionality
  - Query structures by patterns
  - Filter by element names or attributes
  - XPath-like query support
- [ ] **Task 3**: Extract functionality
  - Extract data based on structure patterns
  - Export to various formats (CSV, JSON, XML)
  - Template-based extraction
