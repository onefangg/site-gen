use crate::token::HtmlToken::{BlockQuote, Paragraph, Pre};
use crate::token::PhrasingHtmlContent::{Code, Link, ParagraphPlainText};
use crate::token::MarkdownToken::{Asterik, PlainText, SquareBracketClose};
use crate::token::{MarkdownHeaderToken, HtmlToken, PhrasingHtmlContent, MarkdownToken};
use build_html::Html;

#[derive(Debug)]
pub struct HtmlParser {
    tokens: Vec<MarkdownToken>,
    position: usize,
}

impl HtmlParser {
    pub fn new(tokens: Vec<MarkdownToken>) -> HtmlParser {
        HtmlParser {
            tokens: tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&MarkdownToken> {
        if self.position >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position])
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<&MarkdownToken> {
        if self.position + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position + 1])
        }
    }

    fn peek_ahead(&self, n: usize) -> Option<&MarkdownToken> {
        if self.position + n >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position + n])
        }
    }

    fn parse_plain_text_until(&mut self) -> Vec<u8> {
        let mut parsed_text: Vec<u8> = vec![];
        loop {
            match self.current() {
                None => break,
                Some(PlainText(x)) => {
                    parsed_text.push(*x);
                    self.advance();
                }
                Some(_) => break,
            }
        }
        parsed_text
    }

    fn parse_plain_text_until_specific_token(&mut self, token: MarkdownToken) -> Vec<u8> {
        let mut parsed_text: Vec<u8> = vec![];
        loop {
            if self.current() == Some(&token) {
                break;
            }

            match self.current() {
                None => break,
                Some(PlainText(x)) => {
                    parsed_text.push(*x);
                    self.advance();
                }
                Some(_) => break,
            }
        }
        parsed_text
    }

    fn parse_paragraph(&mut self) -> HtmlToken {
        let mut html_value: Vec<PhrasingHtmlContent> = vec![];

        loop {
            match self.current() {
                None => break,
                Some(MarkdownToken::SquareBracketOpen) => {
                    let mut link_text: Vec<u8> = vec![];
                    let mut link_url: Vec<u8> = vec![];

                    let mut plain_text: Vec<u8> = vec![];

                    self.advance();
                    if let None = self.peek() {
                        html_value.push(ParagraphPlainText(vec![b'[']));
                        break;
                    }
                    loop {
                        match self.current() {
                            None => break,
                            Some(PlainText(x)) => {
                                link_text.push(*x);
                                self.advance();
                            }
                            Some(SquareBracketClose) => {
                                self.advance();
                                break;
                            }
                            Some(_) => {
                                plain_text.push(b'[');
                                plain_text.extend(link_text.clone());
                                link_text.clear();
                                break;
                            }
                        }
                    }
                    if let Some(MarkdownToken::CurveBracketOpen) = self.current() {
                        self.advance();
                        loop {
                            match self.current() {
                                None => break,
                                Some(PlainText(x)) => {
                                    link_url.push(*x);
                                    self.advance();
                                }
                                Some(MarkdownToken::CurveBracketClose) => {
                                    self.advance();
                                    // there is better way to do this
                                    if link_text.len() == 0 {
                                        plain_text.push(b'(');
                                        plain_text.extend(link_url.clone());
                                        link_url.clear();
                                    }
                                    break;
                                }
                                Some(_) => {
                                    plain_text.push(b'(');
                                    plain_text.extend(link_url.clone());
                                    link_url.clear();
                                    break;
                                }
                            }
                        }
                        if link_text.len() == 0 || link_url.len() == 0 {
                            html_value.push(ParagraphPlainText(plain_text));
                        } else {
                            html_value.push(Link(link_text, link_url));
                        }
                    }
                }
                Some(PlainText(c)) | Some(MarkdownToken::Number(c)) => {
                    let mut pt: Vec<u8> = vec![];
                    pt.push(*c);

                    loop {
                        match self.peek() {
                            Some(PlainText(nc)) | Some(MarkdownToken::Number(nc)) => {
                                pt.push(nc.to_owned());
                                self.advance();
                            }
                            None | Some(_) => break,
                        }
                    }
                    html_value.push(ParagraphPlainText(pt));
                    self.advance();
                }
                Some(MarkdownToken::Code) => {
                    let mut pt: Vec<u8> = vec![];
                    loop {
                        match self.peek() {
                            Some(PlainText(nc)) => {
                                pt.push(nc.to_owned());
                                self.advance();
                            }
                            Some(MarkdownToken::Code) => {
                                self.advance();
                                break;
                            }
                            None | Some(_) => break,
                        }
                    }
                    html_value.push(Code(pt));
                    self.advance();
                }
                Some(Asterik) => {
                    let next = self.peek();
                    if let Some(val) = next {
                        if let Asterik = val {
                            // bold case,
                            // html_value.push(Strong(self.parse_text_until()));
                        } else if let PlainText(x) = val {
                            self.advance();
                            let text = self.parse_plain_text_until();

                            if let Some(Asterik) = self.current() {
                                html_value.push(PhrasingHtmlContent::Italic(text));
                            } else {
                                let mut plain_text = vec![b'*'];
                                plain_text.extend(text);
                                html_value.push(ParagraphPlainText(plain_text));
                            }
                            self.advance();
                        }
                    } else {
                        html_value.push(ParagraphPlainText(vec![b'*']));
                        self.advance()
                    }
                }
                Some(_) => break,
            }
        }
        Paragraph(html_value)
    }

    pub fn parse(&mut self) -> Vec<HtmlToken> {
        let mut html_value: Vec<HtmlToken> = vec![];

        loop {
            match self.current() {
                None => break,
                Some(MarkdownToken::Header(headertoken)) => {
                    match &headertoken {
                        MarkdownHeaderToken::Header1(s) => {
                            html_value.push(HtmlToken::Heading1(s.to_owned()));
                        }
                        MarkdownHeaderToken::Header2(s) => {
                            html_value.push(HtmlToken::Heading2(s.to_owned()));
                        }
                        MarkdownHeaderToken::Header3(s) => {
                            html_value.push(HtmlToken::Heading3(s.to_owned()));
                        }
                        MarkdownHeaderToken::Header4(s) => {
                            html_value.push(HtmlToken::Heading4(s.to_owned()));
                        }
                        MarkdownHeaderToken::Header5(s) => {
                            html_value.push(HtmlToken::Heading5(s.to_owned()));
                        }
                        MarkdownHeaderToken::Header6(s) => {
                            html_value.push(HtmlToken::Heading6(s.to_owned()));
                        }
                    }
                    self.advance();
                }
                Some(MarkdownToken::Code) => {
                    if let Some(MarkdownToken::Code) = self.peek()
                        && let Some(MarkdownToken::Code) = self.peek_ahead(2)
                    {
                        // multiline code block
                        self.advance();
                        self.advance();
                        self.advance();

                        let mut block: Vec<u8> = vec![];
                        loop {
                            match self.current() {
                                None => break,
                                Some(val) => {
                                    let s = val.clone().into();
                                    block.push(s);
                                }
                            }

                            if let Some(MarkdownToken::Code) = self.peek()
                                && let Some(MarkdownToken::Code) = self.peek_ahead(2)
                            {
                                html_value.push(Pre(block));
                                self.advance();
                                break;
                            } else {
                                self.advance()
                            }
                        }
                    }
                }
                Some(PlainText(_)) | Some(MarkdownToken::Number(_)) | Some(Asterik) => {
                    html_value.push(self.parse_paragraph());
                    self.advance()
                }
                Some(MarkdownToken::BlockQuote) => {
                    self.advance();
                    let mut whole_text: Vec<u8> = vec![];

                    loop {
                        match self.current() {
                            Some(MarkdownToken::BlockQuote) => {
                                let text = self.parse_plain_text_until_specific_token(MarkdownToken::Newline);
                                whole_text.extend(text);
                                self.advance();
                            }
                            None | Some(_) => {
                                break;
                            }
                        }
                    }
                    let paragraph = self.parse_paragraph();
                    if let Some(content) = paragraph.get_paragraph_contents() {
                        html_value.push(BlockQuote(content));
                    }
                }
                Some(MarkdownToken::Newline) => {
                    // todo - do nothing but move on
                    self.advance();
                }
                Some(_) => {
                    println!("{:?}", self.current());
                    panic!("NOT YET")
                }
            }
        }
        html_value
    }
}

#[cfg(test)]
mod markdown_parser_tests {
    use super::*;
    use crate::token::PhrasingHtmlContent::Italic;
    use crate::token::MarkdownToken::{BlockQuote, SquareBracketOpen};

    #[test]
    fn test_parse_paragraph_for_link() {
        let mut parser = HtmlParser::new(vec![
            MarkdownToken::SquareBracketOpen,
            MarkdownToken::PlainText(b'a'),
            MarkdownToken::SquareBracketClose,
            MarkdownToken::CurveBracketOpen,
            MarkdownToken::PlainText(b'h'),
            MarkdownToken::PlainText(b't'),
            MarkdownToken::CurveBracketClose,
        ]);

        let token = parser.parse_paragraph();
        let expected_token = Paragraph(vec![Link(vec![b'a'], vec![b'h', b't'])]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_paragraph_for_incorrect_link_as_plain_text() {
        let mut parser = HtmlParser::new(vec![
            MarkdownToken::SquareBracketOpen,
            MarkdownToken::PlainText(b'a'),
            MarkdownToken::CurveBracketOpen,
            MarkdownToken::PlainText(b'h'),
            MarkdownToken::PlainText(b't'),
            MarkdownToken::CurveBracketClose,
        ]);

        let token = parser.parse_paragraph();
        let expected_token =
            Paragraph(vec![ParagraphPlainText(vec![b'[', b'a', b'(', b'h', b't'])]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_italic_correct() {
        let mut parser = HtmlParser::new(vec![
            Asterik,
            PlainText(b' '),
            PlainText(b'a'),
            PlainText(b'b'),
            Asterik,
        ]);
        let token = parser.parse_paragraph();
        let expected_token = Paragraph(vec![Italic(vec![b' ', b'a', b'b'])]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_italic_incorrect_parse_as_plain_text() {
        let mut parser = HtmlParser::new(vec![
            Asterik,
            PlainText(b' '),
            PlainText(b'a'),
            PlainText(b'b'),
        ]);
        let token = parser.parse_paragraph();
        let expected_token = Paragraph(vec![ParagraphPlainText(vec![b'*', b' ', b'a', b'b'])]);
        assert_eq!(token, expected_token);
    }
    #[test]
    fn test_parse_italic_incorrect_parse_missing_close_as_plain_text() {
        let mut parser = HtmlParser::new(vec![
            PlainText(b' '),
            PlainText(b'a'),
            PlainText(b'b'),
            Asterik,
        ]);
        let token = parser.parse_paragraph();
        let expected_token = Paragraph(vec![
            ParagraphPlainText(vec![b' ', b'a', b'b']),
            ParagraphPlainText(vec![b'*']),
        ]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_multiline_code_block_correct() {
        let mut parser = HtmlParser::new(vec![
            MarkdownToken::Code,
            MarkdownToken::Code,
            MarkdownToken::Code,
            PlainText(b'd'),
            PlainText(b'e'),
            PlainText(b'f'),
            MarkdownToken::CurveBracketOpen,
            MarkdownToken::CurveBracketClose,
            MarkdownToken::Tab,
            PlainText(b'n'),
            MarkdownToken::Code,
            MarkdownToken::Code,
            MarkdownToken::Code,
        ]);

        let token = parser.parse();
        let expected_token = vec![HtmlToken::Pre(vec![
            b'd', b'e', b'f', b'(', b')', b'\t', b'n',
        ])];
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_block_quote_correct() {
        let mut parser = HtmlParser::new(vec![
            MarkdownToken::BlockQuote,
            PlainText(b'h'),
            PlainText(b'a'),
            PlainText(b' '),
            SquareBracketOpen,
        ]);

        let token = parser.parse();
        let expected_token = vec![HtmlToken::BlockQuote(vec![
            ParagraphPlainText(vec![b'h', b'a', b' ']),
            ParagraphPlainText(vec![b'[']),
        ])];
        assert_eq!(token, expected_token);
    }

    #[test]
    #[ignore = "Refactor first then deal with this"]
    fn test_parse_multiline_block_quote_correct() {
        let mut parser = HtmlParser::new(vec![
            BlockQuote,
            PlainText(b'h'),
            PlainText(b'a'),
            PlainText(b' '),
            MarkdownToken::Newline,
            BlockQuote,
            PlainText(b'a'),
            PlainText(b'b'),
            PlainText(b'c'),
        ]);

        let token = parser.parse();
        let expected_token = vec![HtmlToken::BlockQuote(vec![
            ParagraphPlainText(vec![b'h', b'a', b' ']),
            PhrasingHtmlContent::Break,
            ParagraphPlainText(vec![b'a', b'b', b'c']),
        ])];
        assert_eq!(token, expected_token);
    }
}
