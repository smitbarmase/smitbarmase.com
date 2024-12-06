use std::{fs, path::Path};

use anyhow::Result;
use chrono::NaiveDate;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn format_date(date_str: &str) -> Result<String> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    Ok(date.format("%B %d, %Y").to_string())
}

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    let mut heading_level = 0;
    let mut link_text = String::new();
    let mut link_url = String::new();
    let mut in_link = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading_level = level as usize;
                let prefix = "#".repeat(heading_level);
                html.push_str(&format!(
                    r#"<h{}><span aria-hidden="true">{} </span>"#,
                    heading_level, prefix
                ));
            }
            Event::End(TagEnd::Heading(_)) => {
                html.push_str(&format!("</h{}>", heading_level));
            }
            Event::Start(Tag::List(_)) => {
                html.push_str("<ul>");
            }
            Event::End(TagEnd::List(_)) => {
                html.push_str("</ul>");
            }
            Event::Start(Tag::Item) => {
                html.push_str(r#"<li><span aria-hidden="true">- </span>"#);
            }
            Event::End(TagEnd::Item) => {
                html.push_str("</li>");
            }
            Event::Start(Tag::Paragraph) => {
                html.push_str("<p>");
            }
            Event::End(TagEnd::Paragraph) => {
                html.push_str("</p>");
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                in_link = true;
                link_url = dest_url.to_string();
                link_text.clear();
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                html.push_str(&format!(
                    r#"[{}](<a href="{}">{}</a>)"#,
                    link_text, link_url, link_url
                ));
            }
            Event::Text(text) => {
                if in_link {
                    link_text.push_str(&text);
                } else {
                    html.push_str(&text);
                }
            }
            Event::Code(code) => {
                html.push_str(&format!("<code>{}</code>", code));
            }
            Event::Start(Tag::CodeBlock(_)) => {
                html.push_str("<pre><code>");
            }
            Event::End(TagEnd::CodeBlock) => {
                html.push_str("</code></pre>");
            }
            Event::Start(Tag::Emphasis) => {
                html.push_str("<em>");
            }
            Event::End(TagEnd::Emphasis) => {
                html.push_str("</em>");
            }
            Event::Start(Tag::Strong) => {
                html.push_str("<strong>");
            }
            Event::End(TagEnd::Strong) => {
                html.push_str("</strong>");
            }
            Event::SoftBreak => {
                html.push('\n');
            }
            Event::HardBreak => {
                html.push_str("<br>");
            }
            _ => {}
        }
    }

    html
}
