pub mod html_generator;
pub mod html_parser;
pub mod lexer;
pub mod post_parser;
mod token;

use crate::html_generator::{generate_about_page, generate_home_page, generate_page};
use crate::lexer::tokenize;
use crate::token::{BlogPost, BlogPostMeta};
use html_parser::HtmlParser;
use std::error::Error;
use std::path::Path;
use std::{cmp, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut meta: Vec<BlogPostMeta> = vec![];
    let mut blog_posts: Vec<BlogPost> = vec![];

    // generate page for each posts
    for e in fs::read_dir("./template/posts")? {
        let file_path = e?.path();
        if file_path.is_file() {
            let file = fs::read(file_path)?;
            let (fm, md) = post_parser::parse_markdown(file);
            let result = tokenize(md);
            let mut parser = HtmlParser::new(result.tokens);
            let val = parser.parse();
            let body = BlogPost {
                title: fm.title.clone(),
                date: fm.date.clone(),
                children: val,
            };
            blog_posts.push(body);
        }
    }

    let output_folder = Path::new("./output");
    blog_posts.sort_by_key(|x| cmp::Reverse(x.date));
    for (i, body) in blog_posts.iter().enumerate() {
        let generated_html_body = generate_page(body.to_owned())?;
        let path = format!("post_{}.html", i);
        fs::write(output_folder.join(path.clone()), generated_html_body)?;
        meta.push(BlogPostMeta {
            title: body.title.to_string(),
            date: body.date,
            link: path,
        });
    }

    // generate about page
    let about_fp = fs::read("./template/about.md")?;
    let about_result = tokenize(about_fp);
    let mut parser = HtmlParser::new(about_result.tokens);
    let val = parser.parse();
    let about_html_body = generate_about_page(val)?;
    fs::write(output_folder.join("about.html"), about_html_body)?;

    // generate home page
    let index = generate_home_page(meta);
    fs::write(output_folder.join("index.html"), index?)?;
    Ok(())
}
