use crate::token::{HtmlBody, HtmlToken, PhrasingHtmlContent};
use build_html::{Html, HtmlElement, HtmlTag};
use std::error::Error;



pub fn generate(parsedTags: HtmlBody) -> Result<String, Box<dyn Error>> {
    let mut build_element = HtmlElement::new(HtmlTag::Div);

    for ele in parsedTags.children {
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
                let first = t.first();
                if let Some(val) = first {
                    match val {
                        PhrasingHtmlContent::ParagraphPlainText(s) => {
                            let mut parent_para_element = HtmlElement::new(HtmlTag::ParagraphText)
                                .with_child(str::from_utf8(&s)?.into());

                            for inline in t[1..].iter() {
                                match inline {
                                    PhrasingHtmlContent::ParagraphPlainText(p) => {
                                        parent_para_element.add_child(
                                            HtmlElement::new(HtmlTag::ParagraphText)
                                                .with_child(str::from_utf8(&p)?.into())
                                                .into(),
                                        );
                                    }
                                    PhrasingHtmlContent::Strong(p) => {
                                        parent_para_element.add_child(
                                            HtmlElement::new(HtmlTag::Strong)
                                                .with_child(str::from_utf8(&p)?.into())
                                                .into(),
                                        );
                                    }
                                    PhrasingHtmlContent::Italic(p) => {
                                        parent_para_element.add_child(
                                            HtmlElement::new(HtmlTag::Italic)
                                                .with_child(str::from_utf8(&p)?.into())
                                                .into(),
                                        );
                                    }
                                    PhrasingHtmlContent::Code(p) => {
                                        parent_para_element.add_child(
                                            HtmlElement::new(HtmlTag::CodeText)
                                                .with_child(str::from_utf8(&p)?.into())
                                                .into(),
                                        );
                                    }
                                    PhrasingHtmlContent::Link(p, l) => {
                                        parent_para_element.add_child(
                                            HtmlElement::new(HtmlTag::Link)
                                                .with_attribute("href", str::from_utf8(&l)?)
                                                .with_child(str::from_utf8(&p)?.into())
                                                .into(),
                                        );
                                    }
                                }
                            }
                            build_element.add_child(parent_para_element.into())
                        }
                        _ => { panic!("Top most paragraph must be a p tag") }
                    }
                }
            }
            HtmlToken::Article(_) => {
                unimplemented!("shouldn't happen yet");
            }
        }
    }

    Ok(build_element.to_html_string())
}
