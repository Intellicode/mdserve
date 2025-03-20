use crate::markdown::render_markdown;
use crate::utils::etag::generate_etag;
use axum::http::{HeaderMap, Response, StatusCode, header};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn serve_markdown(path: &Path, template: &str, headers: &HeaderMap) -> Response<String> {
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
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| "# Error\nFailed to read file.".to_string());
    let html = render_markdown(&content, template);

    // Build response with ETag
    let mut builder = Response::builder().header(header::CONTENT_TYPE, "text/html");

    if let Some(etag) = etag {
        builder = builder.header(header::ETAG, etag);
    }

    builder.body(html.0).unwrap()
}

pub fn export_markdown_to_html(
    input_dir: &Path,
    output_dir: &Path,
    template: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    // Iterate over markdown files in the input directory and subdirectories
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Read markdown content
            let content = fs::read_to_string(&path)?;
            let html = render_markdown(&content, template);

            // Determine output file path
            let relative_path = path.strip_prefix(input_dir)?;
            let mut output_path = output_dir.join(relative_path);
            output_path.set_extension("html");

            // Create parent directories if they don't exist
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Write HTML content to output file
            fs::write(output_path, html.0)?;
        }
    }

    Ok(())
}
