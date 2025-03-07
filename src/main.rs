mod handlers;
mod markdown;
mod server;
mod utils;

use crate::handlers::markdown_handler::export_markdown_to_html;
use server::Server;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 && args[1] == "export" {
        let input_dir = PathBuf::from(&args[2]);
        let output_dir = PathBuf::from(&args[3]);
        let template = include_str!("../templates/markdown.html");
        export_markdown_to_html(&input_dir, &output_dir, template)?;
        println!("Exported markdown files from {} to {}", args[2], args[3]);
        return Ok(());
    }

    let server = Server::new();
    server.run().await
}
