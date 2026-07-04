use crate::token::HtmlToken::Paragraph;
use crate::token::PhrasingHtmlContent::{Code, Link, ParagraphPlainText};
use crate::token::Token::{Asterik, PlainText, SquareBracketClose};
use crate::token::{HeaderToken, HtmlToken, PhrasingHtmlContent, Token};
use build_html::Html;

#[derive(Debug)]
pub struct MarkdownParser {
    tokens: Vec<Token>,
    position: usize,
}

impl MarkdownParser {
    pub fn new(tokens: Vec<Token>) -> MarkdownParser {
        MarkdownParser {
            tokens: tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        if self.position >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position])
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<&Token> {
        if self.position + 1 >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.position + 1])
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

    fn parse_paragraph(&mut self) -> HtmlToken {
        let mut html_value: Vec<PhrasingHtmlContent> = vec![];

        loop {
            match self.current() {
                None => break,
                Some(Token::SquareBracketOpen) => {
                    let mut link_text: Vec<u8> = vec![];
                    let mut link_url: Vec<u8> = vec![];

                    let mut plain_text: Vec<u8> = vec![];

                    self.advance();
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
                    if let Some(Token::CurveBracketOpen) = self.current() {
                        self.advance();
                        loop {
                            match self.current() {
                                None => break,
                                Some(PlainText(x)) => {
                                    link_url.push(*x);
                                    self.advance();
                                }
                                Some(Token::CurveBracketClose) => {
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
                Some(PlainText(c)) | Some(Token::Number(c)) => {
                    let mut pt: Vec<u8> = vec![];
                    pt.push(*c);

                    loop {
                        match self.peek() {
                            Some(PlainText(nc)) | Some(Token::Number(nc)) => {
                                pt.push(nc.to_owned());
                                self.advance();
                            }
                            None | Some(_) => break,
                        }
                    }
                    html_value.push(ParagraphPlainText(pt));
                    self.advance();
                }
                Some(Token::Code) => {
                    let mut pt: Vec<u8> = vec![];
                    loop {
                        match self.peek() {
                            Some(PlainText(nc)) => {
                                pt.push(nc.to_owned());
                                self.advance();
                            }
                            Some(Token::Code) => {
                                self.advance();
                                break;
                            }
                            None | Some(_) => break,
                        }
                    }
                    html_value.push(Code(pt));
                    self.advance();
                }
                Some(Token::Asterik) => {
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
                Some(Token::Header(headertoken)) => {
                    match &headertoken {
                        HeaderToken::Header1(s) => {
                            html_value.push(HtmlToken::Heading1(s.to_owned()));
                        }
                        HeaderToken::Header2(s) => {
                            html_value.push(HtmlToken::Heading2(s.to_owned()));
                        }
                        HeaderToken::Header3(s) => {
                            html_value.push(HtmlToken::Heading3(s.to_owned()));
                        }
                        HeaderToken::Header4(s) => {
                            html_value.push(HtmlToken::Heading4(s.to_owned()));
                        }
                        HeaderToken::Header5(s) => {
                            html_value.push(HtmlToken::Heading5(s.to_owned()));
                        }
                        HeaderToken::Header6(s) => {
                            html_value.push(HtmlToken::Heading6(s.to_owned()));
                        }
                    }
                    self.advance();
                }
                Some(PlainText(_)) | Some(Token::Number(_)) | Some(Asterik) => {
                    html_value.push(self.parse_paragraph());
                    self.advance()
                }
                Some(Token::Newline) => {
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

    #[test]
    fn test_parse_paragraph_for_link() {
        let mut parser = MarkdownParser::new(vec![
            Token::SquareBracketOpen,
            Token::PlainText(b'a'),
            Token::SquareBracketClose,
            Token::CurveBracketOpen,
            Token::PlainText(b'h'),
            Token::PlainText(b't'),
            Token::CurveBracketClose,
        ]);

        let token = parser.parse_paragraph();
        let expected_token = Paragraph(vec![Link(vec![b'a'], vec![b'h', b't'])]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_paragraph_for_incorrect_link_as_plain_text() {
        let mut parser = MarkdownParser::new(vec![
            Token::SquareBracketOpen,
            Token::PlainText(b'a'),
            Token::CurveBracketOpen,
            Token::PlainText(b'h'),
            Token::PlainText(b't'),
            Token::CurveBracketClose,
        ]);

        let token = parser.parse_paragraph();
        let expected_token =
            Paragraph(vec![ParagraphPlainText(vec![b'[', b'a', b'(', b'h', b't'])]);
        assert_eq!(token, expected_token);
    }

    #[test]
    fn test_parse_italic_correct() {
        let mut parser = MarkdownParser::new(vec![
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
        let mut parser = MarkdownParser::new(vec![
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
        let mut parser = MarkdownParser::new(vec![
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
}
