use crate::token::{HeaderToken, Token};

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

    fn peek_ahead(&self, peek_ahead: usize) -> Option<Vec<u8>> {
        if self.position + peek_ahead >= self.input.len() {
            None
        } else {
            Some(self.input[(self.position + 1)..(self.position + peek_ahead + 1)].to_vec())
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

pub fn tokenize(input: Vec<u8>) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

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
                    // this may or may not sit here
                    match header_ok {
                        1 => tokens.push(Token::Header(HeaderToken::Header1(read_header))),
                        2 => tokens.push(Token::Header(HeaderToken::Header2(read_header))),
                        3 => tokens.push(Token::Header(HeaderToken::Header3(read_header))),
                        4 => tokens.push(Token::Header(HeaderToken::Header4(read_header))),
                        5 => tokens.push(Token::Header(HeaderToken::Header5(read_header))),
                        6 => tokens.push(Token::Header(HeaderToken::Header6(read_header))),
                        _ => panic!(">7 # not allowed for headers"),
                    }
                } else {
                    tokens.push(Token::PlainText(b'#'));
                }
                lexer.advance();
            }
            Some(b'`') => {
                tokens.push(Token::Code);
                lexer.advance();
            }
            // not handling the case where two or more spaces are treated as a new line
            Some(b'\n') | Some(b'\r') => {
                tokens.push(Token::Newline);
                lexer.advance();
            }
            Some(b'>') => {
                tokens.push(Token::BlockQuote);
                lexer.advance();
            }
            Some(b'*') => {
                tokens.push(Token::Asterik);
                lexer.advance();
            }
            Some(b'\t') => {
                tokens.push(Token::Tab);
                lexer.advance();
            }
            Some(b'-') | Some(b'+') => {
                tokens.push(Token::UnorderedList);
                lexer.advance();
            }
            Some(b'[') => {
                tokens.push(Token::SquareBracketOpen);
                lexer.advance();
            }
            Some(b']') => {
                tokens.push(Token::SquareBracketClose);
                lexer.advance();
            }
            Some(b'(') => {
                tokens.push(Token::CurveBracketOpen);
                lexer.advance();
            }
            Some(b')') => {
                tokens.push(Token::CurveBracketClose);
                lexer.advance();
            }
            Some(i @ b'0'..=b'9') => {
                // most :/ implementation, handwavy probably doesnt work
                tokens.push(Token::Number(i));
                lexer.advance();
            }
            Some(x) => {
                tokens.push(Token::PlainText(x));
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
            Token::Header(HeaderToken::Header1("Header 1".into())),
            Token::Header(HeaderToken::Header2("a".into())),
            Token::Header(HeaderToken::Header3("bc".into())),
            Token::Header(HeaderToken::Header4("d".into())),
            Token::Header(HeaderToken::Header5("e".into())),
            Token::Header(HeaderToken::Header6("6".into())),
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
            Token::PlainText(b'#'),
            Token::PlainText(b'H'),
            Token::PlainText(b' '),
            Token::Number(b'1'),
        ];
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens, expected_output);
    }
}
