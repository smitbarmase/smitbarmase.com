use crate::content::Post;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub author: &'a str,
    pub posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub author: &'a str,
    pub date: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct SitemapTemplate<'a> {
    pub site_url: &'a str,
    pub posts: &'a [Post],
}
