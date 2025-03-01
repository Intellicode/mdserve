use crate::markdown::render_markdown;
use axum::response::Html;
use std::path::PathBuf;

pub async fn serve_markdown(path: PathBuf, template: String) -> Html<String> {
    let content =
        std::fs::read_to_string(&path).unwrap_or_else(|_| "# Error\nFile not found.".to_string());
    render_markdown(&content, template)
}
