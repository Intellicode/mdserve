use axum::response::Html;
use pulldown_cmark::{Options, Parser, html};

pub fn render_markdown(content: &str, template: String) -> Html<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Html(template.replace("{}", &html_output))
}
