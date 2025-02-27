use axum::{
    Router,
    response::Html,
    routing::{get, get_service},
};
use pulldown_cmark::{Options, Parser, html};
use std::env;
use std::fs;
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get directory path from command line args
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

    // Load template at startup
    let template = include_str!("../templates/markdown.html");

    // Handler for serving index.md
    async fn serve_index(dir: PathBuf, template: String) -> Html<String> {
        let index_path = dir.join("index.md");
        let content = fs::read_to_string(index_path)
            .unwrap_or_else(|_| "# Welcome\nIndex file not found.".to_string());

        // Set up options for GitHub-flavored markdown
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);

        // Parse and render markdown
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Html(template.replace("{}", &html_output))
    }

    // Handler for serving markdown files
    async fn serve_markdown(path: PathBuf, template: String) -> Html<String> {
        let content =
            fs::read_to_string(&path).unwrap_or_else(|_| "# Error\nFile not found.".to_string());

        // Set up options for GitHub-flavored markdown
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);

        // Parse and render markdown
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Html(template.replace("{}", &html_output))
    }

    // Build our application with routes
    let serve_dir = dir.clone();
    let markdown_dir = dir.clone();
    let template = template.to_string(); // Convert to owned String
    let template_index = template.clone();
    let app = Router::new()
        .route("/", get(move || serve_index(dir, template_index)))
        .route(
            "/*path",
            get(move |path: axum::extract::Path<String>| {
                let mut path = markdown_dir.join(path.0);
                let template = template.clone();
                async move {
                    if path.to_str().map_or(false, |s| s.ends_with('/')) {
                        path.push("index.md");
                    }

                    if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                        serve_markdown(path, template).await
                    } else {
                        Html("Not found".to_string())
                    }
                }
            }),
        )
        .fallback_service(get_service(ServeDir::new(serve_dir)));

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
