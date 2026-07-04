pub mod html_generator;
pub mod lexer;
pub mod markdown_parser;
mod token;

use crate::lexer::tokenize;
use crate::token::HtmlBody;
use markdown_parser::MarkdownParser;
use std::error::Error;
use std::fs;
use crate::html_generator::generate;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read("min2.md")?;
    let result = tokenize(file);
    let mut parser = MarkdownParser::new(result);
    // println!("{:#?}", parser);
    let val = parser.parse();
    println!("{:#?}", val);
    let body = HtmlBody { children: val };
    let generated_html_body = generate(body)?;
    println!("{}", generated_html_body);

    Ok(())
}
