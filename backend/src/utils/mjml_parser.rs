use mrml;
use anyhow::{self, Error};

pub fn mjml_to_html(input: String) -> Result<String, Error> {
    // parse string to mjml...
    let parsed_template_html = mrml::parse(&input)
        .map_err(|e| anyhow::anyhow!(format!("Failed to parse MJML template: {e}")))?;

    let opts = mrml::prelude::render::Options::default();
    let parsed_html = parsed_template_html
        .render(&opts)
        .map_err(|e| anyhow::anyhow!(format!("Failed to render MJML to HTML: {e}")))?;

    Ok(parsed_html)
}