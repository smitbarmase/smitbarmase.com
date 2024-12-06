use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site: SiteConfig,
    pub paths: PathsConfig,
}

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub author: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    pub posts: String,
}

impl Config {
    pub fn load(path: &str) -> Config {
        let content =
            fs::read_to_string(path).expect(&format!("Failed to read config file: {}", path));
        toml::from_str(&content).expect("Failed to parse config.toml")
    }
}
