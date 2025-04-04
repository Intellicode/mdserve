use crate::config::Config;
use serde::Serialize;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct TemplateData<'a> {
    pub content: &'a str,
    pub title: &'a str,
    pub header_title: &'a str,
    pub description: &'a str,
    pub frontmatter_block: &'a str,
    pub custom_css: &'a str,
    pub custom_header: &'a str,
    pub custom_footer: &'a str,
    pub navigation_links: &'a str,
}

// Create a new template renderer instance
fn create_template_renderer(template_path: Option<&Path>) -> Result<Tera, String> {
    let mut tera = if let Some(path) = template_path {
        if path.exists() && path.is_file() {
            // Use custom template file
            let template_name = path.file_name().unwrap().to_str().unwrap();
            let content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read template file: {}", e))?;

            let mut t = Tera::default();
            t.add_raw_template(template_name, &content)
                .map_err(|e| format!("Failed to add template: {}", e))?;
            t
        } else {
            return Err(format!(
                "Template path does not exist or is not a file: {:?}",
                path
            ));
        }
    } else {
        // Use default templates directory
        match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };

    // Add any custom filters or functions here
    tera.autoescape_on(vec![]); // Disable autoescaping for HTML content

    Ok(tera)
}

// Keep the initialize_templates function for backward compatibility
// but it now does nothing since we don't use static templates anymore
pub fn initialize_templates(_template_path: Option<&Path>) -> Result<(), String> {
    // This function is kept for backward compatibility
    // It now does nothing since we create a template renderer every time we need one
    Ok(())
}

pub fn render(
    template_name: &str,
    content: &str,
    title: &str,
    header_title: &str,
    description: &str,
    frontmatter_block: &str,
    config: Option<&Config>,
) -> Result<String, String> {
    // Create a new template renderer every time
    let templates = create_template_renderer(None)?;

    let mut context = Context::new();
    context.insert("content", content);
    context.insert("title", title);
    context.insert("header_title", header_title);
    context.insert("description", description);
    context.insert("frontmatter_block", frontmatter_block);

    // Default values
    let default_nav = r#"<a href="/" style="color: var(--link-color); text-decoration: none; font-size: 1.1rem;">Home</a>
        <a href="/docs" style="color: var(--link-color); text-decoration: none; font-size: 1.1rem;">Documentation</a>
        <a href="/about" style="color: var(--link-color); text-decoration: none; font-size: 1.1rem;">About</a>"#;
    let navigation_links = default_nav;

    // Add config-based customizations
    if let Some(cfg) = config {
        // Build navigation links
        if let Some(navigation) = &cfg.navigation {
            let mut nav_links = String::new();
            for link in navigation {
                nav_links.push_str(&format!(
                    "<a href=\"{}\" style=\"color: var(--link-color); text-decoration: none; font-size: 1.1rem;\">{}</a>",
                    link.url, link.text
                ));
            }
            context.insert("navigation_links", &nav_links);
        } else {
            context.insert("navigation_links", navigation_links);
        }
    } else {
        // No config, use defaults
        context.insert("navigation_links", navigation_links);
    }

    templates
        .render(template_name, &context)
        .map_err(|e| format!("Template rendering error: {}", e))
}
