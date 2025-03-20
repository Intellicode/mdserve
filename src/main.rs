mod handlers;
mod markdown;
mod server;
mod utils;

use crate::handlers::markdown_handler::export_markdown_to_html;
use clap::{Parser, Subcommand};
use server::Server;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "mdserve")]
#[command(about = "A markdown server and exporter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Directory to serve (backward compatibility mode)
    #[arg(global = false)]
    directory: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serve markdown files from a directory
    Serve {
        /// Directory containing markdown files to serve
        directory: PathBuf,
    },

    /// Export markdown files to HTML
    Export {
        /// Input directory containing markdown files
        input_dir: PathBuf,

        /// Output directory for HTML files
        output_dir: PathBuf,

        /// Optional custom HTML template file
        #[arg(long)]
        template: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Handle commands
    match &cli.command {
        Some(Commands::Serve { directory }) => {
            // Validate directory
            if !directory.exists() || !directory.is_dir() {
                error!(
                    "Directory does not exist or is not a directory: {}",
                    directory.display()
                );
                return Ok(());
            }

            start_server(directory).await?;
        }
        Some(Commands::Export {
            input_dir,
            output_dir,
            template,
        }) => {
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
            let template_content = match template {
                Some(path) => {
                    if !path.exists() || !path.is_file() {
                        error!(
                            "Template file does not exist or is not a file: {}",
                            path.display()
                        );
                        return Ok(());
                    }
                    match fs::read_to_string(path) {
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

            export_markdown_to_html(input_dir, output_dir, &template_content)?;
            info!(
                "Exported markdown files from {} to {}",
                input_dir.display(),
                output_dir.display()
            );
        }
        None => {
            // Backward compatibility mode - direct directory argument
            if let Some(directory) = &cli.directory {
                if !directory.exists() || !directory.is_dir() {
                    error!(
                        "Directory does not exist or is not a directory: {}",
                        directory.display()
                    );
                    return Ok(());
                }

                start_server(directory).await?;
            } else {
                // No arguments provided - show help
                let _ = Cli::parse_from(["mdserve", "--help"]);
            }
        }
    }

    Ok(())
}

async fn start_server(directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new_with_directory(directory.to_path_buf());
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
    }
    Ok(())
}
