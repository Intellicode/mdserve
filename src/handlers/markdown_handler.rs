use crate::markdown::render_markdown;
use crate::utils::etag::generate_etag;
use axum::http::{HeaderMap, Response, StatusCode, header};
use std::path::PathBuf;

pub fn serve_markdown(path: &PathBuf, template: String, headers: HeaderMap) -> Response<String> {
    // Generate ETag for the file
    let etag = generate_etag(path);

    // Check if the file exists and handle not found case
    if !path.exists() {
        let content = "# Error\nFile not found.";
        let html = render_markdown(content, template);
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(html.0)
            .unwrap();
    }

    // Check if-none-match header
    if let (Some(etag_str), Some(if_none_match)) = (&etag, headers.get(header::IF_NONE_MATCH)) {
        if if_none_match == etag_str {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .body(String::new())
                .unwrap();
        }
    }

    // Read and render content
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| "# Error\nFailed to read file.".to_string());
    let html = render_markdown(&content, template);

    // Build response with ETag
    let mut builder = Response::builder().header(header::CONTENT_TYPE, "text/html");

    if let Some(etag) = etag {
        builder = builder.header(header::ETAG, etag);
    }

    builder.body(html.0).unwrap()
}
