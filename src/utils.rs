use std::{fs, path::Path};

use anyhow::Result;
use chrono::NaiveDate;
use regex::Regex;

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

pub fn stylize_html(html_content: &str) -> String {
    let mut result = html_content.to_string();

    // Transform links: <a href="url">text</a> → [text](<a href="url">url</a>)
    let link_re = Regex::new(r#"<a\s+([^>]*href="([^"]+)"[^>]*)>([^<]+)</a>"#).unwrap();
    result = link_re
        .replace_all(&result, |caps: &regex::Captures| {
            let attrs = &caps[1];
            let url = &caps[2];
            let text = &caps[3];
            format!(r#"[{}](<a {}>{}</a>)"#, text, attrs, url)
        })
        .to_string();

    // Transform list items: <li> → <li><span aria-hidden="true">- </span>
    let li_re = Regex::new(r"<li>").unwrap();
    result = li_re
        .replace_all(&result, r#"<li><span aria-hidden="true">- </span>"#)
        .to_string();

    // Transform headings: <h1> → <h1><span aria-hidden="true"># </span>
    for level in 1..=6 {
        let heading_re = Regex::new(&format!(r"<h{}>", level)).unwrap();
        let prefix = "#".repeat(level);
        result = heading_re
            .replace_all(
                &result,
                &format!(r#"<h{}><span aria-hidden="true">{} </span>"#, level, prefix),
            )
            .to_string();
    }

    result
}
