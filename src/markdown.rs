use axum::response::Html;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
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

pub fn render_markdown(content: &str, template: &str) -> Html<String> {
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

    // Replace content placeholder
    let mut final_html = template.replace("{{content}}", &html_output);

    // Default values
    let default_title = "Markdown Viewer";
    let default_description = "Markdown document";
    let default_header_title = "Wiki";

    // Replace placeholders with frontmatter data
    if let Some(fm) = frontmatter {
        // Title handling
        let title = fm.title.as_deref().unwrap_or(default_title);
        final_html = final_html.replace("{{title}}", title);
        final_html = final_html.replace("{{header_title}}", title);

        // Description handling
        let description = fm.description.as_deref().unwrap_or(default_description);
        final_html = final_html.replace("{{description}}", description);

        // Create frontmatter info block
        let mut frontmatter_html = String::new();
        frontmatter_html.push_str("<div class=\"frontmatter-info\">");

        // Add author and date if available
        if fm.author.is_some() || fm.date.is_some() {
            frontmatter_html.push_str("<div>");

            if let Some(author) = &fm.author {
                frontmatter_html.push_str(&format!("<span class=\"author\">By {}</span>", author));
            }

            if let Some(date) = &fm.date {
                if fm.author.is_some() {
                    frontmatter_html.push_str(" on ");
                }
                frontmatter_html.push_str(&format!("<span class=\"date\">{}</span>", date));
            }

            frontmatter_html.push_str("</div>");
        }

        // Add description if available
        if let Some(description) = &fm.description {
            frontmatter_html.push_str(&format!("<div class=\"description\">{}</div>", description));
        }

        // Add tags if available
        if let Some(tags) = &fm.tags {
            if !tags.is_empty() {
                frontmatter_html.push_str("<div class=\"tags\">");
                for tag in tags {
                    frontmatter_html.push_str(&format!("<span class=\"tag\">{}</span> ", tag));
                }
                frontmatter_html.push_str("</div>");
            }
        }

        frontmatter_html.push_str("</div>");

        final_html = final_html.replace("{{frontmatter_block}}", &frontmatter_html);
    } else {
        // No frontmatter, use defaults
        final_html = final_html.replace("{{title}}", default_title);
        final_html = final_html.replace("{{header_title}}", default_header_title);
        final_html = final_html.replace("{{description}}", default_description);
        final_html = final_html.replace("{{frontmatter_block}}", "");
    }

    Html(final_html)
}
