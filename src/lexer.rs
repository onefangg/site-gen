use crate::token::{MarkdownHeaderToken, MarkdownToken};

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
}

impl Lexer {
    fn new(input: Vec<u8>) -> Lexer {
        Lexer {
            input: input,
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn move_back(&mut self) { self.position -= 1; }

    fn current(&self) -> Option<u8> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    fn peek_ahead(&self, peek_ahead: usize) -> Option<u8> {
        if self.position + peek_ahead >= self.input.len() {
            None
        } else {
            Some(self.input[(self.position + peek_ahead)])
        }
    }

    fn read_text(&mut self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();
        loop {
            match self.current() {
                None | Some(b'`') | Some(b'\r') | Some(b'\n') | Some(b'*') | Some(b'[')
                | Some(b']') | Some(b'(') | Some(b')') => break,
                Some(x) => {
                    text.push(x);
                    self.advance();
                }
            }
        }
        text
    }

    fn read_header(&mut self) -> Option<usize> {
        let mut count = 0usize;
        loop {
            match self.current() {
                None => break,
                Some(b'#') => {
                    count += 1;
                    self.advance();
                }
                Some(b' ') => break,
                Some(_) => {
                    // unwind count advanced [
                    for _ in 0..count {
                        self.move_back()
                    }
                    return None
                },
            }
        }

        Some(count)
    }
}

pub fn tokenize(input: Vec<u8>) -> Vec<MarkdownToken> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<MarkdownToken> = Vec::new();

    loop {
        match lexer.current() {
            None => {
                break tokens;
            }
            Some(b'#') => {
                let header_count = lexer.read_header();
                if let Some(header_ok) = header_count {
                    lexer.advance();
                    let read_header = lexer.read_text();
                    match header_ok {
                        1 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header1(read_header))),
                        2 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header2(read_header))),
                        3 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header3(read_header))),
                        4 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header4(read_header))),
                        5 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header5(read_header))),
                        6 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header6(read_header))),
                        _ => panic!(">7 # not allowed for headers"),
                    }
                } else {
                    tokens.push(MarkdownToken::PlainText(b'#'));
                }
                lexer.advance();
            }
            Some(b'`') => {
                tokens.push(MarkdownToken::Code);
                lexer.advance();
            }
            // not handling the case (yet) where two or more spaces are treated as a new line
            Some(b'\n') | Some(b'\r') => {
                tokens.push(MarkdownToken::Newline);
                lexer.advance();
            }
            Some(b'>') => {
                tokens.push(MarkdownToken::BlockQuote);
                lexer.advance();
            }
            Some(b'*') => {
                tokens.push(MarkdownToken::Asterik);
                lexer.advance();
            }
            Some(b'\t') => {
                tokens.push(MarkdownToken::Tab);
                lexer.advance();
            }
            Some(b'-') => {
                tokens.push(MarkdownToken::Dash);
                lexer.advance();
            }
            Some(b'[') => {
                tokens.push(MarkdownToken::SquareBracketOpen);
                lexer.advance();
            }
            Some(b']') => {
                tokens.push(MarkdownToken::SquareBracketClose);
                lexer.advance();
            }
            Some(b'(') => {
                tokens.push(MarkdownToken::CurveBracketOpen);
                lexer.advance();
            }
            Some(b')') => {
                tokens.push(MarkdownToken::CurveBracketClose);
                lexer.advance();
            }
            Some(i @ b'0'..=b'9') => {
                tokens.push(MarkdownToken::Number(i));
                lexer.advance();
            }
            Some(x) => {
                tokens.push(MarkdownToken::PlainText(x));
                lexer.advance();
            }
        }
    }
}

#[cfg(test)]
mod tokenize_tests {
    use super::*;
    use crate::token::HtmlToken;

    #[test]
    fn tokenize_headers() {
        let input = "# Header 1
## a
### bc
#### d
##### e
###### 6"
            .as_bytes()
            .to_vec();

        let expected_output = vec![
            MarkdownToken::Header(MarkdownHeaderToken::Header1("Header 1".into())),
            MarkdownToken::Header(MarkdownHeaderToken::Header2("a".into())),
            MarkdownToken::Header(MarkdownHeaderToken::Header3("bc".into())),
            MarkdownToken::Header(MarkdownHeaderToken::Header4("d".into())),
            MarkdownToken::Header(MarkdownHeaderToken::Header5("e".into())),
            MarkdownToken::Header(MarkdownHeaderToken::Header6("6".into())),
        ];
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens, expected_output);
    }

    #[test]
    fn tokenize_invalid_headers_as_plain_text() {
        let input = "#H 1"
            .as_bytes()
            .to_vec();

        let expected_output = vec![
            MarkdownToken::PlainText(b'#'),
            MarkdownToken::PlainText(b'H'),
            MarkdownToken::PlainText(b' '),
            MarkdownToken::Number(b'1'),
        ];
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens, expected_output);
    }
}
