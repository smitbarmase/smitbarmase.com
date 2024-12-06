use std::fs::read_to_string;
use std::path::Path;

use anyhow::{Context, Result};
use gray_matter::{engine::YAML, Matter};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::utils::{format_date, markdown_to_html};

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub content: String,
}

pub fn get_posts(content_dir: &str) -> Result<Vec<Post>> {
    let mut posts: Vec<Post> = Vec::new();
    let matter = Matter::<YAML>::new();

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let path = entry.path();

        let parsed = matter.parse(&read_to_string(path)?);
        let frontmatter: Frontmatter = parsed
            .data
            .context("Missing frontmatter")?
            .deserialize()
            .context("Failed to parse frontmatter")?;

        let slug = Path::new(path)
            .file_stem()
            .context("Invalid file name")?
            .to_str()
            .context("Invalid UTF-8 in file name")?
            .to_string();

        posts.push(Post {
            slug,
            title: frontmatter.title,
            date: format_date(&frontmatter.date)?,
            content: markdown_to_html(&parsed.content),
        });
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}
