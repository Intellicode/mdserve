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

    // Handler for serving index.md
    async fn serve_index(dir: PathBuf) -> Html<String> {
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

        // Wrap in HTML template with some GFM-friendly styling
        let html = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Markdown Viewer</title>
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Source+Serif+4:ital,wght@0,400;0,600;1,400;1,600&display=swap" rel="stylesheet">
                <style>
                    :root {{
                        --text-color: #1a1a1a;
                        --background: #ffffff;
                        --code-bg: #f8fafc;
                        --border-color: #e5e7eb;
                        --link-color: #2b4b9a;
                    }}
                    
                    body {{ 
                        max-width: 70ch;
                        margin: 0 auto; 
                        padding: 2rem;
                        font-family: 'Source Serif 4', Georgia, 'Computer Modern', serif;
                        font-size: 1.125rem;
                        line-height: 1.75;
                        color: var(--text-color);
                        background: var(--background);
                        text-rendering: optimizeLegibility;
                        -webkit-font-smoothing: antialiased;
                        -moz-osx-font-smoothing: grayscale;
                    }}

                    h1, h2, h3, h4, h5, h6 {{
                        margin-top: 2.5em;
                        margin-bottom: 0.75em;
                        line-height: 1.2;
                        font-weight: 600;
                    }}

                    h1 {{
                        font-size: 2.5rem;
                        margin-top: 0;
                        text-align: center;
                    }}

                    h2 {{
                        font-size: 1.75rem;
                        margin-top: 2em;
                    }}

                    h3 {{ font-size: 1.5rem; }}
                    h4 {{ font-size: 1.25rem; }}

                    p {{
                        margin: 1.5em 0;
                        text-align: justify;
                        hyphens: auto;
                    }}

                    a {{
                        color: var(--link-color);
                        text-decoration: none;
                    }}

                    a:hover {{
                        text-decoration: underline;
                    }}

                    code {{
                        font-family: 'Computer Modern Typewriter', 'Courier New', monospace;
                        font-size: 0.9em;
                        padding: 0.2em 0.4em;
                        background: var(--code-bg);
                        border-radius: 2px;
                    }}

                    pre {{
                        background: var(--code-bg);
                        padding: 1.25rem;
                        border-radius: 4px;
                        overflow-x: auto;
                        border: 1px solid var(--border-color);
                    }}

                    pre code {{
                        padding: 0;
                        background: none;
                    }}

                    blockquote {{
                        margin: 1.5em 0;
                        padding: 0.5em 1.5em;
                        border-left: 3px solid #999;
                        font-style: italic;
                    }}

                    table {{
                        width: 100%;
                        border-collapse: collapse;
                        margin: 2em 0;
                        font-size: 0.95em;
                    }}

                    th, td {{
                        padding: 0.75em;
                        border: 1px solid var(--border-color);
                    }}

                    th {{
                        font-weight: 600;
                        background: var(--code-bg);
                    }}

                    ul, ol {{
                        padding-left: 1.5em;
                        margin: 1.5em 0;
                    }}

                    li {{
                        margin: 0.5em 0;
                    }}

                    hr {{
                        border: none;
                        border-top: 1px solid var(--border-color);
                        margin: 2em 0;
                    }}

                    img {{
                        max-width: 100%;
                        height: auto;
                        border-radius: 8px;
                    }}

                    .task-list-item {{
                        list-style-type: none;
                        margin-left: -1.5em;
                    }}

                    .task-list-item input {{
                        margin-right: 0.5em;
                    }}

                    @media (max-width: 768px) {{
                        body {{
                            padding: 1rem;
                        }}
                        
                        h1 {{ font-size: 2rem; }}
                        h2 {{ font-size: 1.5rem; }}
                        h3 {{ font-size: 1.25rem; }}
                    }}
                </style>
            </head>
            <body>
                {}
            </body>
            </html>
        "#,
            html_output
        );

        Html(html)
    }

    // Build our application with routes
    let serve_dir = dir.clone();
    let app = Router::new()
        .route("/", get(move || serve_index(dir)))
        .fallback_service(get_service(ServeDir::new(serve_dir)));

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
