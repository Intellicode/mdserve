mod config;
mod handlers;
mod markdown;
mod server;
mod template;
mod utils;

use crate::config::Config;
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

    /// Config file path (backward compatibility mode)
    #[arg(long, global = false)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serve markdown files from a directory
    Serve {
        /// YAML config file path
        #[arg(long)]
        config: Option<PathBuf>,
    },

    /// Export markdown files to HTML
    Export {
        /// Output directory for HTML files
        output_dir: PathBuf,

        /// YAML config file path
        #[arg(long)]
        config: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Handle commands
    match &cli.command {
        Some(Commands::Serve { config }) => {
            // Load config if provided
            let config_path = config.clone();

            // If no config provided, use default directory
            if config_path.is_none() {
                error!("No config file specified. Please provide a config file with --config");
                return Ok(());
            }

            start_server(&PathBuf::from("."), config_path).await?;
        }
        Some(Commands::Export { output_dir, config }) => {
            // Load config if provided
            let config_path = config.clone();

            if config_path.is_none() {
                error!("No config file specified. Please provide a config file with --config");
                return Ok(());
            }

            let config_obj = Config::from_file(&config_path.unwrap());

            // Use input_dir from config
            let source_dir = config_obj.get_source_directory();

            // Validate directories
            if !source_dir.exists() || !source_dir.is_dir() {
                error!(
                    "Input directory does not exist or is not a directory: {}",
                    source_dir.display()
                );
                return Ok(());
            }

            // Determine template path from config
            let template_path = if let Some(tpl_dir) = &config_obj.template_dir {
                // Try to find layout.html in the template directory
                let default_template = tpl_dir.join("layout.html");
                if default_template.exists() && default_template.is_file() {
                    Some(default_template)
                } else {
                    None
                }
            } else {
                None
            };

            // Initialize templates
            if let Err(e) = template::initialize_templates(template_path.as_deref()) {
                error!("Failed to initialize templates: {}", e);
                return Ok(());
            }

            // Get template content - either from file or use the default
            let template_content = match &template_path {
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
                    include_str!("../templates/layout.html").to_string()
                }
            };

            export_markdown_to_html(&source_dir, output_dir, &template_content)?;
            info!(
                "Exported markdown files from {} to {}",
                source_dir.display(),
                output_dir.display()
            );
        }
        None => {
            // Backward compatibility mode - direct config argument
            if let Some(config) = &cli.config {
                start_server(&PathBuf::from("."), Some(config.clone())).await?;
            } else {
                // No arguments provided - show help
                let _ = Cli::parse_from(["mdserve", "--help"]);
            }
        }
    }

    Ok(())
}

async fn start_server(
    _: &Path, // Unused parameter as we'll only use config
    config_path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Default directory is "." but will be overridden by config
    let server = Server::new_with_directory(PathBuf::from(".")).with_config(config_path);

    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
    }
    Ok(())
}
