mod cli;
mod processor;
mod utils;
mod xsconfig;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use is_terminal::IsTerminal;
use processor::{create_progress_bar, print_summary, process_xml_files, write_result_to_file};
use std::time::Instant;
use tracing::info;
use utils::{find_xml_files, init_logging, validate_directory};
use xsconfig::XsConfig;

fn main() -> Result<()> {
    // Per rust-cli-recommendations, explicitly control color output.
    // If we are not in an interactive terminal, disable colors for the `colored` crate.
    if !std::io::stdout().is_terminal() {
        colored::control::set_override(false);
    }

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config = if std::path::Path::new(&cli.config).exists() {
        XsConfig::from_file(&cli.config)
            .with_context(|| format!("Failed to load config from {}", cli.config))?
    } else {
        eprintln!("⚠️  Config file not found, using defaults");
        XsConfig::default()
    };

    // Merge CLI overrides
    let mut config = config.merge_with_cli(cli.output.clone(), cli.threads);

    // Override max_depth if provided via CLI
    if let Some(max_depth) = cli.max_depth {
        config.processing.max_depth = max_depth;
    }

    // Override log level
    config.logging.level = cli.effective_log_level();

    // Override pretty print
    if cli.no_pretty {
        config.output.pretty_print = false;
    }

    // Initialize logging
    let log_file = config.log_file_path();
    init_logging(&config.logging.level, log_file.as_deref())
        .context("Failed to initialize logging")?;

    info!("🚀 XML Structure Analyzer starting...");
    info!("Input directory: {}", cli.input_dir.display());
    info!("Output file: {}", config.output.output_file);

    // Validate input directory
    validate_directory(&cli.input_dir).context("Input directory validation failed")?;

    // Configure rayon thread pool if specified
    if config.processing.num_threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(config.processing.num_threads)
            .build_global()
            .context("Failed to configure thread pool")?;
        info!("Using {} threads", config.processing.num_threads);
    } else {
        info!("Using auto-detected thread count");
    }

    let start_time = Instant::now();

    // Find all XML files
    info!("🔍 Scanning for XML files...");
    let xml_files = find_xml_files(
        &cli.input_dir,
        &config.processing.file_extensions,
        config.processing.max_depth,
    )
    .context("Failed to find XML files")?;

    info!("Found {} XML files", xml_files.len());

    // Create progress bar
    let progress_bar = if !cli.no_progress {
        Some(create_progress_bar(xml_files.len()))
    } else {
        None
    };

    // Process files
    info!("⚙️  Processing XML files...");
    let result =
        process_xml_files(xml_files, progress_bar).context("Failed to process XML files")?;

    // Write results
    let output_path = config.output_file_path();
    write_result_to_file(&result, &output_path, config.output.pretty_print)
        .context("Failed to write results")?;

    // Print summary
    print_summary(&result);

    let elapsed = start_time.elapsed();
    println!("\n⏱️  Total time: {:.2}s", elapsed.as_secs_f64());
    println!("✅ Results saved to: {}", output_path.display());

    info!("Processing completed successfully");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_main_compiles() {
        // This test just ensures the main function compiles
        assert!(true);
    }
}
