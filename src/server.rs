use crate::handlers::markdown_handler;
use axum::{
    Router,
    routing::{get, get_service},
};
use std::{env, path::PathBuf};
use tower_http::services::ServeDir;

pub struct Server {
    dir: PathBuf,
    template: String,
    port: String,
}

impl Server {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dir = Self::get_directory()?;
        let template = include_str!("../templates/markdown.html").to_string();
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

        Ok(Self {
            dir,
            template,
            port,
        })
    }

    fn get_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("Usage: {} <directory>", args[0]);
            std::process::exit(1);
        }

        let dir = PathBuf::from(&args[1]);
        if !dir.is_dir() {
            eprintln!("Error: {} is not a directory", args[1]);
            std::process::exit(1);
        }

        Ok(dir)
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        self.print_startup_message(&addr);

        let md_dir_index = self.dir.clone();
        let md_dir_path = self.dir.clone();
        let serve_dir_static = self.dir;
        let template_index = self.template.clone();
        let template_path = self.template;

        let app = Router::new()
            .route(
                "/",
                get(move || {
                    markdown_handler::serve_markdown(md_dir_index.join("index.md"), template_index)
                }),
            )
            .route(
                "/*path",
                get(move |path| {
                    markdown_handler::handle_markdown_path(path, md_dir_path, template_path)
                }),
            )
            .fallback_service(get_service(ServeDir::new(serve_dir_static)));

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

    fn print_startup_message(&self, addr: &str) {
        println!(
            "\x1b[32m
╔═══════════════════════════════════════╗
║                                       ║
║   🚀 Markdown Server is running!      ║
║   ➜ http://{:<27}║
║                                       ║
╚═══════════════════════════════════════╝
\x1b[0m",
            addr
        );
    }
}
