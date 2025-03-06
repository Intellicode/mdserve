use crate::handlers::markdown_handler;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{
    Router,
    routing::{get, get_service},
};
use chrono::Utc;
use dashmap::DashMap;
use serde_json::json;
use std::sync::Arc;
use std::{env, path::PathBuf, time::Instant};
use tower_http::services::ServeDir;

pub struct Server {
    dir: PathBuf,
    template: String,
    port: String,
}

struct AppState {
    cache: DashMap<String, Response<String>>,
    dir: PathBuf,
    template: String,
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
        let cache: DashMap<String, Response<String>> = DashMap::new();
        let shared_state = Arc::new(AppState {
            cache,
            dir: md_dir_index,
            template: self.template,
        });
        let app = Router::new()
            .route("/", get(handler_index))
            .route("/*path", get(handler_all))
            .fallback_service(get_service(ServeDir::new(self.dir)))
            .with_state(shared_state)
            .layer(axum::middleware::from_fn(request_logger));

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await.map_err(|e| {
            eprintln!("Server error: {e}");
            Box::new(e) as Box<dyn std::error::Error>
        })
    }

    fn print_startup_message(&self, addr: &str) {
        println!(
            "\x1b[32m
╔═══════════════════════════════════════╗
║                                       ║
║   🚀 Markdown Server is running!      ║
║   ➜ http://{addr:<27}║
║                                       ║
╚═══════════════════════════════════════╝
\x1b[0m"
        );
    }
}

async fn handler_index(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response<String> {
    let file = "index.md";
    handle(file, &state, headers)
}

async fn handler_all(
    Path(filename): Path<String>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response<String> {
    let file_including_index = if filename.ends_with('/') {
        format!("{filename}/index.md")
    } else {
        filename
    };
    handle(&file_including_index, &state, headers)
}

// handle
fn handle(filename: &str, state: &Arc<AppState>, headers: HeaderMap) -> Response<String> {
    let cache_key = filename;
    if let Some(cached_html) = state.cache.get(cache_key) {
        return cached_html.clone();
    }
    let rendered = markdown_handler::serve_markdown(
        &state.dir.join(filename),
        state.template.clone(),
        headers,
    );
    state.cache.insert(cache_key.to_string(), rendered.clone());
    rendered
}

async fn request_logger(req: Request<Body>, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let timestamp = Utc::now().to_rfc3339();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let log_entry = json!({
        "timestamp": timestamp,
        "method": method.to_string(),
        "uri": uri.to_string(),
        "duration_ms": duration.as_millis(),
        "status": response.status().as_u16()
    });
    println!("{log_entry}");

    response
}
