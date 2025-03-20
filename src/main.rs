mod handlers;
mod markdown;
mod server;
mod utils;

use crate::handlers::markdown_handler::export_markdown_to_html;
use server::Server;
use std::env;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    // Handle export command
    if args.len() >= 4 && args[1] == "export" {
        let mut input_dir = PathBuf::new();
        let mut output_dir = PathBuf::new();
        let mut template_file: Option<PathBuf> = None;

        let mut i = 2;
        while i < args.len() {
            if args[i] == "--template" {
                if i + 1 < args.len() {
                    template_file = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    error!("--template flag requires a file path");
                    return Ok(());
                }
            } else if input_dir.as_os_str().is_empty() {
                input_dir = PathBuf::from(&args[i]);
                i += 1;
            } else if output_dir.as_os_str().is_empty() {
                output_dir = PathBuf::from(&args[i]);
                i += 1;
            } else {
                i += 1;
            }
        }

        // Check if required directories are provided
        if input_dir.as_os_str().is_empty() || output_dir.as_os_str().is_empty() {
            error!("Both input and output directories are required for export");
            error!("Usage: mdserve export <input_dir> <output_dir> [--template <template_file>]");
            return Ok(());
        }

        // Validate directories
        if !input_dir.exists() || !input_dir.is_dir() {
            error!(
                "Input directory does not exist or is not a directory: {}",
                input_dir.display()
            );
            return Ok(());
        }

        if !output_dir.exists() || !output_dir.is_dir() {
            error!(
                "Output directory does not exist or is not a directory: {}",
                output_dir.display()
            );
            return Ok(());
        }

        // Get template content - either from file or use the default
        let template = match template_file {
            Some(path) => {
                if !path.exists() || !path.is_file() {
                    error!(
                        "Template file does not exist or is not a file: {}",
                        path.display()
                    );
                    return Ok(());
                }
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        info!("Using custom template file: {}", path.display());
                        content
                    }
                    Err(e) => {
                        error!("Failed to read template file: {}", e);
                        return Ok(());
                    }
                }
            }
            None => {
                info!("Using default template");
                include_str!("../templates/markdown.html").to_string()
            }
        };

        export_markdown_to_html(&input_dir, &output_dir, &template)?;
        info!(
            "Exported markdown files from {} to {}",
            input_dir.display(),
            output_dir.display()
        );
        return Ok(());
    }

    // Start the server
    let server = Server::new();
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
    }
    Ok(())
}
