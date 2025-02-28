mod handlers;
mod markdown;
mod server;
mod utils;

use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new()?;
    server.run().await
}
