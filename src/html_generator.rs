use crate::token::{BlogPost, BlogPostMeta, HtmlToken, PhrasingHtmlContent};
use build_html::{Html, HtmlChild, HtmlContainer, HtmlElement, HtmlPage, HtmlTag};
use std::error::Error;

fn parse_format(
    mut parent: HtmlElement,
    t: PhrasingHtmlContent,
) -> Result<HtmlElement, Box<dyn Error>> {
    match t {
        PhrasingHtmlContent::ParagraphPlainText(p) => {
            Ok(parent.with_child(str::from_utf8(&p)?.into()))
        }
        PhrasingHtmlContent::Strong(p) => {
            parent.add_child(
                HtmlElement::new(HtmlTag::Strong)
                    .with_child(str::from_utf8(&p)?.into())
                    .into(),
            );
            Ok(parent)
        }
        PhrasingHtmlContent::Italic(p) => {
            parent.add_child(
                HtmlElement::new(HtmlTag::Italic)
                    .with_child(str::from_utf8(&p)?.into())
                    .into(),
            );
            Ok(parent)
        }
        PhrasingHtmlContent::Code(p) => {
            parent.add_child(
                HtmlElement::new(HtmlTag::CodeText)
                    .with_attribute("class", "code-block")
                    .with_child(str::from_utf8(&p)?.into())
                    .into(),
            );
            Ok(parent)
        }
        PhrasingHtmlContent::Link(p, l) => {
            parent.add_child(
                HtmlElement::new(HtmlTag::Link)
                    .with_attribute("href", str::from_utf8(&l)?)
                    .with_child(str::from_utf8(&p)?.into())
                    .into(),
            );
            Ok(parent)
        }
        PhrasingHtmlContent::Break => {
            parent.add_child(HtmlElement::new(HtmlTag::LineBreak).into());
            Ok(parent)
        }
    }
}

fn generate_header(parsed: BlogPost) -> Result<HtmlElement, Box<dyn Error>> {
    let mut header = HtmlElement::new(HtmlTag::Header);
    header.add_child(
        HtmlElement::new(HtmlTag::Heading1)
            .with_child(str::from_utf8(parsed.title.as_ref())?.into())
            .into(),
    );
    header.add_child(
        HtmlElement::new(HtmlTag::ParagraphText)
            .with_child(format!("Updated: {}", parsed.date.format("%d %b %Y - %H:%M:%S")).into())
            .into(),
    );

    Ok(header)
}

fn generate_navbar() -> Result<HtmlElement, Box<dyn Error>> {
    let mut navbar = HtmlElement::new(HtmlTag::Navigation).with_attribute("class", "navbar");
    let mut navlinks = HtmlElement::new(HtmlTag::UnorderedList);
    navlinks.add_child(
        HtmlElement::new(HtmlTag::ListElement)
            .with_child(
                HtmlElement::new(HtmlTag::Link)
                    .with_attribute("href", "./index.html")
                    .with_child("Home".into())
                    .into(),
            )
            .into(),
    );
    navlinks.add_child(
        HtmlElement::new(HtmlTag::ListElement)
            .with_child(
                HtmlElement::new(HtmlTag::Link)
                    .with_attribute("href", "./about.html")
                    .with_child("About".into())
                    .into(),
            )
            .into(),
    );
    navbar.add_child(navlinks.into());
    Ok(navbar)
}

fn generate_article(parsed_body: Vec<HtmlToken>) -> Result<HtmlElement, Box<dyn Error>> {
    let mut build_element = HtmlElement::new(HtmlTag::Article);
    for ele in parsed_body {
        match ele {
            HtmlToken::Heading1(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading1)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Heading2(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading2)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Heading3(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading3)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Heading4(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading4)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Heading5(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading5)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Heading6(t) => {
                build_element.add_child(
                    HtmlElement::new(HtmlTag::Heading6)
                        .with_child(str::from_utf8(&t)?.into())
                        .into(),
                );
            }
            HtmlToken::Paragraph(t) => {
                let mut parent_para_element = HtmlElement::new(HtmlTag::ParagraphText);

                for ele in t {
                    parent_para_element = parse_format(parent_para_element, ele)?;
                }
                build_element.add_child(parent_para_element.into())
            }
            HtmlToken::BlockQuote(content) => {
                let mut parent = HtmlElement::new(HtmlTag::Blockquote);
                for e in content {
                    parent = parse_format(parent, e)?;
                }
                build_element.add_child(parent.into())
            }
            HtmlToken::Pre(t) => {
                let parent = HtmlElement::new(HtmlTag::PreformattedText);
                build_element.add_child(parse_format(parent, t)?.into());
            }
            HtmlToken::Break | HtmlToken::Article(_) => {
                // do nothing
            }
        }
    }
    Ok(build_element)
}

fn create_base_page() -> Result<HtmlPage, Box<dyn Error>> {
    Ok(HtmlPage::new()
        .with_meta(vec![("charset", "utf-8")]).with_raw("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")
        .with_meta(vec![("description", "Personal Blog for Software and among other things - Full-Stack Software Engineer / Developer based in Singapore")])
        .with_title("onefangg blog")
        .with_style(include_str!("../template/styles.css"))
        .with_html(generate_navbar()?))
}

pub fn generate_page(parsed_tags: BlogPost) -> Result<String, Box<dyn Error>> {
    let header_content = generate_header(parsed_tags.clone())?;
    let body_content = generate_article(parsed_tags.children)?;
    let page = create_base_page()?
        .with_html(header_content)
        .with_html(body_content);
    Ok(page.to_html_string())
}

pub fn generate_about_page(parsed_tags: Vec<HtmlToken>) -> Result<String, Box<dyn Error>> {
    let page = create_base_page()?;
    let body_content = generate_article(parsed_tags)?;
    Ok(page.with_html(body_content).to_html_string())
}

// take in an input of posts generated and the link
pub fn generate_home_page(posts: Vec<BlogPostMeta>) -> Result<String, Box<dyn Error>> {
    let page = create_base_page()?;
    let mut container = HtmlElement::new(HtmlTag::Div).with_attribute("class", "post-container");

    for p in posts {
        let post_link = HtmlElement::new(HtmlTag::Link)
            .with_attribute("href", p.link)
            .with_attribute("class", "post-link")
            .with_child(
                HtmlChild::Element(
                    HtmlElement::new(HtmlTag::Heading1)
                        .with_child(p.title.into()),
                )
                    .into(),
            )
            .with_child(
                HtmlChild::Element(
                    HtmlElement::new(HtmlTag::ParagraphText)
                        .with_child(p.date.format("%d %b %Y").to_string().into()),
                )
                    .into(),
            );
        container = container.with_child(post_link.into());
    }
    Ok(page.with_html(container).to_html_string())
}
