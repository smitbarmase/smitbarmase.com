mod config;
mod content;
mod templates;
mod utils;

use askama::Template;
use std::fs;
use std::path::Path;

use config::Config;
use content::get_posts;
use templates::{IndexTemplate, PostTemplate, SitemapTemplate};

use crate::utils::copy_dir_recursive;

const CONTENT_DIR: &str = "content";
const DIST_DIR: &str = "dist";
const PUBLIC_DIR: &str = "public";

fn main() {
    println!("Loading configuration");
    let config_path = Path::new(CONTENT_DIR).join("config.toml");
    let config = Config::load(config_path.to_str().expect("Invalid UTF-8 in config path"));

    println!("Parsing content files");
    let posts_path = Path::new(CONTENT_DIR)
        .join(&config.paths.posts)
        .canonicalize()
        .expect("Failed to resolve posts path");
    let posts = get_posts(posts_path.to_str().expect("Invalid UTF-8 in posts path"))
        .expect("Failed to parse content files");

    println!("Setting up dist directory");
    let dist_path = Path::new(DIST_DIR);
    if dist_path.exists() {
        fs::remove_dir_all(dist_path).expect("Failed to clean dist directory");
    }
    fs::create_dir_all(dist_path).expect("Failed to create dist directory");

    println!("Copying public assets");
    let public_path = Path::new(PUBLIC_DIR);
    if public_path.exists() {
        copy_dir_recursive(public_path, dist_path).expect("Failed to copy public assets");
    }

    println!("Generating pages");

    // Generate post pages
    for post in &posts {
        let template = PostTemplate {
            description: &config.site.description,
            author: &config.site.author,
            title: &post.title,
            date: &post.date,
            content: &post.content,
        };
        let output = template.render().expect("Failed to render post template");
        let output_path = dist_path.join(format!("{}.html", post.slug));
        fs::write(&output_path, output).expect("Failed to write post file");
    }

    // Generate index page
    let index_template = IndexTemplate {
        title: &config.site.title,
        description: &config.site.description,
        author: &config.site.author,
        posts: &posts,
    };
    let index_output = index_template
        .render()
        .expect("Failed to render index template");
    fs::write(dist_path.join("index.html"), index_output).expect("Failed to write index file");

    println!("Generating sitemap");
    let sitemap_template = SitemapTemplate {
        site_url: &config.site.url,
        posts: &posts,
    };
    let sitemap_output = sitemap_template
        .render()
        .expect("Failed to render sitemap template");
    fs::write(dist_path.join("sitemap.xml"), sitemap_output).expect("Failed to write sitemap");

    println!("Build complete. Generated {} posts + index", posts.len());
}
