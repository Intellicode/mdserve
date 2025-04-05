use crate::config::Config;
use crate::template;
use axum::response::Html;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn extract_frontmatter(content: &str) -> (Option<Frontmatter>, &str) {
    if content.starts_with("---\n") || content.starts_with("---\r\n") {
        if let Some(end_index) = content[3..].find("\n---\n") {
            let yaml_end = end_index + 3;
            let frontmatter_str = &content[3..yaml_end];
            let remaining_content = &content[(yaml_end + 4)..];
            match serde_yaml::from_str::<Frontmatter>(frontmatter_str) {
                Ok(frontmatter) => return (Some(frontmatter), remaining_content),
                Err(_) => return (None, content),
            }
        } else if let Some(end_index) = content[3..].find("\r\n---\r\n") {
            let yaml_end = end_index + 5;
            let frontmatter_str = &content[3..yaml_end];
            let remaining_content = &content[(yaml_end + 6)..];
            match serde_yaml::from_str::<Frontmatter>(frontmatter_str) {
                Ok(frontmatter) => return (Some(frontmatter), remaining_content),
                Err(_) => return (None, content),
            }
        }
    }
    (None, content)
}

// Extract the parsed components from markdown content
pub fn parse_markdown(content: &str) -> (String, String, String, String, String) {
    // Extract frontmatter if present
    let (frontmatter, content_without_frontmatter) = extract_frontmatter(content);

    // Process markdown content
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(content_without_frontmatter, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Default values
    let default_title = "Markdown Viewer";
    let default_description = "Markdown document";
    let default_header_title = "Wiki";

    // Build frontmatter HTML block and collect metadata for template
    let (title, header_title, description, frontmatter_html) = if let Some(fm) = frontmatter {
        // Title handling
        let title = fm.title.as_deref().unwrap_or(default_title).to_string();
        let header_title = title.clone();

        // Description handling
        let description = fm
            .description
            .as_deref()
            .unwrap_or(default_description)
            .to_string();

        // Create frontmatter info block
        let mut frontmatter_html = String::new();
        frontmatter_html.push_str("<div class=\"frontmatter-info\">");

        // Add author and date if available
        if fm.author.is_some() || fm.date.is_some() {
            frontmatter_html.push_str("<div>");
            if let Some(author) = &fm.author {
                frontmatter_html.push_str(&format!("<span class=\"author\">By {author}</span>"));
            }
            if let Some(date) = &fm.date {
                if fm.author.is_some() {
                    frontmatter_html.push_str(" on ");
                }
                frontmatter_html.push_str(&format!("<span class=\"date\">{date}</span>"));
            }
            frontmatter_html.push_str("</div>");
        }

        // Add description if available
        if let Some(description) = &fm.description {
            frontmatter_html.push_str(&format!("<div class=\"description\">{description}</div>"));
        }

        // Add tags if available
        if let Some(tags) = &fm.tags {
            if !tags.is_empty() {
                frontmatter_html.push_str("<div class=\"tags\">");
                for tag in tags {
                    frontmatter_html.push_str(&format!("<span class=\"tag\">{tag}</span> "));
                }
                frontmatter_html.push_str("</div>");
            }
        }
        frontmatter_html.push_str("</div>");

        (title, header_title, description, frontmatter_html)
    } else {
        // No frontmatter, use defaults
        (
            default_title.to_string(),
            default_header_title.to_string(),
            default_description.to_string(),
            String::new(),
        )
    };

    (
        html_output,
        title,
        header_title,
        description,
        frontmatter_html,
    )
}

pub fn render_markdown(content: &str, config: Option<&Config>) -> Html<String> {
    // Parse markdown and extract components
    let (html_output, title, header_title, description, frontmatter_html) = parse_markdown(content);

    // Use Tera template for rendering
    let template_name = "layout.html";
    let result = template::render(
        template_name,
        &html_output,
        &title,
        &header_title,
        &description,
        &frontmatter_html,
        config,
    );

    match result {
        Ok(html_string) => Html(html_string),
        Err(err) => Html(format!("<h1>Template Error</h1><p>{}</p>", err)),
    }
}
