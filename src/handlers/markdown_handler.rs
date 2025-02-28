use crate::markdown::render_markdown;
use axum::{extract::Path, response::Html};
use std::path::PathBuf;

pub async fn serve_markdown(path: PathBuf, template: String) -> Html<String> {
    let content =
        std::fs::read_to_string(&path).unwrap_or_else(|_| "# Error\nFile not found.".to_string());
    render_markdown(&content, template)
}

pub async fn handle_markdown_path(
    Path(path): Path<String>,
    markdown_dir: PathBuf,
    template: String,
) -> Html<String> {
    let mut path = markdown_dir.join(path);

    if path.to_str().is_some_and(|s| s.ends_with('/')) {
        path.push("index.md");
    }

    if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
        serve_markdown(path, template).await
    } else {
        Html("Not found".to_string())
    }
}
