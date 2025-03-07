mod handlers;
mod markdown;
mod server;
mod utils;

use crate::handlers::markdown_handler::export_markdown_to_html;
use server::Server;
use std::env;
use std::path::PathBuf;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    // Handle export command
    if args.len() == 4 && args[1] == "export" {
        let input_dir = PathBuf::from(&args[2]);
        let output_dir = PathBuf::from(&args[3]);

        // Validate directories
        if !input_dir.exists() || !input_dir.is_dir() {
            error!(
                "Input directory does not exist or is not a directory: {}",
                args[2]
            );
            return Ok(());
        }

        if !output_dir.exists() || !output_dir.is_dir() {
            error!(
                "Output directory does not exist or is not a directory: {}",
                args[3]
            );
            return Ok(());
        }

        let template = include_str!("../templates/markdown.html");
        export_markdown_to_html(&input_dir, &output_dir, template)?;
        info!("Exported markdown files from {} to {}", args[2], args[3]);
        return Ok(());
    }

    // Start the server
    let server = Server::new();
    if let Err(e) = server.run().await {
        error!("Server error: {}", e);
    }
    Ok(())
}
