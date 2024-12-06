mod config;
mod content;
mod templates;
mod utils;

use askama::Template;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use config::Config;
use content::get_posts;
use templates::{IndexTemplate, PostTemplate, SitemapTemplate};

use crate::utils::copy_dir_recursive;

fn main() {
    let pb = ProgressBar::new(6);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Step 1: Load configuration
    pb.set_message("Loading configuration");
    let config = Config::load("content/config.toml");
    pb.inc(1);

    // Step 2: Parse content files
    pb.set_message("Parsing content files");
    let posts = get_posts(&config.paths.posts).expect("Failed to parse content files");
    pb.inc(1);

    // Step 3: Setup dist directory
    pb.set_message("Setting up dist directory");
    let dist_path = Path::new("dist");
    if dist_path.exists() {
        fs::remove_dir_all(dist_path).expect("Failed to clean dist directory");
    }
    fs::create_dir_all(dist_path).expect("Failed to create dist directory");
    pb.inc(1);

    // Step 4: Copy public assets
    pb.set_message("Copying public assets");
    let public_path = Path::new("public");
    if public_path.exists() {
        copy_dir_recursive(public_path, dist_path).expect("Failed to copy public assets");
    }
    pb.inc(1);

    // Step 5: Generate pages
    pb.set_message("Generating pages");

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
    pb.inc(1);

    // Step 6: Generate sitemap
    pb.set_message("Generating sitemap");
    let sitemap_template = SitemapTemplate {
        site_url: &config.site.url,
        posts: &posts,
    };
    let sitemap_output = sitemap_template
        .render()
        .expect("Failed to render sitemap template");
    fs::write(dist_path.join("sitemap.xml"), sitemap_output).expect("Failed to write sitemap");
    pb.inc(1);

    pb.finish_with_message(format!(
        "Build complete! Generated {} posts + index",
        posts.len()
    ));
}
