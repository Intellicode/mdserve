use crate::config::Config;
use serde::Serialize;
use std::{error::Error, path::Path};
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct TemplateData<'a> {
    pub content: &'a str,
    pub title: &'a str,
    pub header_title: &'a str,
    pub description: &'a str,
    pub frontmatter_block: &'a str,
    pub base_url: &'a str,
}

impl TemplateData<'_> {
    // Add a method to render HTML with a provided template
    pub fn to_html(&self, template_content: &str) -> (String, String) {
        // Create a temporary file for our main template
        let temp_dir = std::env::temp_dir().join("mdserve_templates");
        let _ = std::fs::create_dir_all(&temp_dir);
        let temp_template_path = temp_dir.join("main_template.html");
        if let Err(e) = std::fs::write(&temp_template_path, template_content) {
            return (
                format!(
                    "<h1>Template Error</h1><p>Failed to create temporary template file: {}</p>",
                    e
                ),
                "".to_string(),
            );
        }

        // Get the template directory from the current working directory
        let template_dir = std::path::PathBuf::from("./templates");

        // Create a proper Tera environment that can handle includes
        let template_pattern = format!("{}/**/*.html", template_dir.display());

        let mut tera = match Tera::new(&template_pattern) {
            Ok(t) => t,
            Err(e) => {
                return (
                    format!(
                        "<h1>Template Error</h1><p>Failed to initialize Tera with templates: {}</p>",
                        e
                    ),
                    "".to_string(),
                );
            }
        };

        // Also add our temporary template
        if let Err(e) = tera.add_template_file(&temp_template_path, Some("main_template")) {
            return (
                format!(
                    "<h1>Template Error</h1><p>Failed to add temporary template: {}</p>",
                    e
                ),
                "".to_string(),
            );
        }

        // Set up the context
        let mut context = Context::new();
        context.insert("content", self.content);
        context.insert("title", self.title);
        context.insert("header_title", self.header_title);
        context.insert("description", self.description);
        context.insert("frontmatter_block", self.frontmatter_block);
        context.insert("base_url", self.base_url);
        // Add empty navigation_links by default
        context.insert("navigation_links", "");

        // Render the template
        match tera.render("main_template", &context) {
            Ok(html) => {
                // Clean up the temporary file
                let _ = std::fs::remove_file(temp_template_path);
                (html, "".to_string())
            }
            Err(e) => {
                // Clean up the temporary file
                let _ = std::fs::remove_file(temp_template_path);
                // Log details for debugging
                println!("Template error: {}", e);
                println!("Error kind: {:?}", e.kind);
                println!("Source: {:?}", e.source());
                (
                    format!(
                        "<h1>Template Error</h1><p>{}</p><pre>{:?}</pre><p>This error typically occurs when template includes cannot be resolved.</p>",
                        e,
                        e.source()
                    ),
                    "".to_string(),
                )
            }
        }
    }
}

// Create a new template renderer instance
fn create_template_renderer(
    template_path: Option<&Path>,
    config: Option<&Config>,
) -> Result<Tera, String> {
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
    } else if let Some(cfg) = config {
        // Try to use template directory from config
        if let Some(template_dir) = cfg.get_template_directory() {
            let template_pattern = format!("{}/**/*.html", template_dir.display());
            match Tera::new(&template_pattern) {
                Ok(t) => t,
                Err(e) => {
                    return Err(format!(
                        "Failed to load templates from {}: {}",
                        template_dir.display(),
                        e
                    ));
                }
            }
        } else {
            // Use default template directory
            match Tera::new("templates/**/*.html") {
                Ok(t) => t,
                Err(e) => {
                    return Err(format!("Failed to load default templates: {}", e));
                }
            }
        }
    } else {
        // Use default templates directory (no config provided)
        match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                return Err(format!("Failed to load default templates: {}", e));
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
    // Create a new template renderer every time, passing config for template directory
    let templates = create_template_renderer(None, config)?;
    let mut context = Context::new();
    context.insert("content", content);
    context.insert("title", title);
    context.insert("header_title", header_title);
    context.insert("description", description);
    context.insert("frontmatter_block", frontmatter_block);

    // Default values
    let base_url = "/";

    // Add config-based customizations
    if let Some(cfg) = config {
        // Get base URL from config
        let base_url_from_config = cfg.get_base_url();
        context.insert("base_url", &base_url_from_config);

        // Build navigation links
        if let Some(navigation) = &cfg.navigation {
            let mut nav_links = String::new();
            for link in navigation {
                let url = if link.url.starts_with("http") || link.url.starts_with("https") {
                    // External URL, use as is
                    link.url.clone()
                } else {
                    // Internal URL, prepend base_url if it doesn't start with /
                    if link.url.starts_with("/") {
                        format!("{}{}", base_url_from_config.trim_end_matches('/'), link.url)
                    } else {
                        format!(
                            "{}/{}",
                            base_url_from_config.trim_end_matches('/'),
                            link.url
                        )
                    }
                };

                nav_links.push_str(&format!(
                    "<a href=\"{}\" style=\"color: var(--link-color); text-decoration: none; font-size: 1.1rem;\">{}</a>",
                    url, link.text
                ));
            }
            context.insert("navigation_links", &nav_links);
        }
    } else {
        // No config, use defaults
        context.insert("base_url", base_url);
    }

    templates
        .render(template_name, &context)
        .map_err(|e| format!("Template rendering error: {}", e))
}
