use crate::token::MarkdownToken::{CodeBlock, MultilineCode, PlainText};
use crate::token::{MarkdownHeaderToken, MarkdownInformation, MarkdownToken};

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

    fn move_back(&mut self) {
        self.position -= 1;
    }

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
            Some(self.input[self.position + peek_ahead])
        }
    }

    fn peek_back(&self, size: i32) -> Option<u8> {
        let idx = self.position as i32 + size;

        if self.position + idx as usize >= self.input.len() {
            None
        } else {
            Some(self.input[idx as usize])
        }
    }

    fn read_text(&mut self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();
        loop {
            match self.current() {
                None | Some(b'`') | Some(b'\r') | Some(b'\n') | Some(b'*') | Some(b'[')
                | Some(b']') | Some(b'(') | Some(b')') | Some(b'<') | Some(b'>') => break,
                Some(x) => {
                    text.push(x);
                    self.advance();
                }
            }
        }
        text
    }

    fn read_code_text(&mut self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();
        loop {
            match self.current() {
                None | Some(b'`') => break,
                Some(x) => {
                    text.push(x);
                    self.advance();
                }
            }
        }
        text
    }

    fn read_line(&mut self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();
        loop {
            match self.current() {
                None | Some(b'\n') | Some(b'\r') => break,
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
                    return None;
                }
            }
        }

        Some(count)
    }
}

pub fn tokenize(input: Vec<u8>) -> MarkdownInformation {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<MarkdownToken> = Vec::new();
    loop {
        match lexer.current() {
            None => {
                break;
            }
            Some(b'#') => {
                let header_count = lexer.read_header();
                if let Some(header_ok) = header_count {
                    lexer.advance();
                    let read_header = lexer.read_text();
                    match header_ok {
                        1 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header1(
                            read_header,
                        ))),
                        2 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header2(
                            read_header,
                        ))),
                        3 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header3(
                            read_header,
                        ))),
                        4 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header4(
                            read_header,
                        ))),
                        5 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header5(
                            read_header,
                        ))),
                        6 => tokens.push(MarkdownToken::Header(MarkdownHeaderToken::Header6(
                            read_header,
                        ))),
                        _ => panic!(">7 # not allowed for headers"),
                    }
                } else {
                    tokens.push(MarkdownToken::PlainText(b'#'));
                }
                lexer.advance();
            }
            Some(b'`') => {
                if let Some(b'`') = lexer.peek()
                    && let Some(b'`') = lexer.peek_ahead(2)
                {
                    lexer.advance();
                    lexer.advance();
                    lexer.advance();
                    let lang = lexer.read_text();
                    let code_block = lexer.read_code_text();

                    if let Some(b'`') = lexer.current()
                        && let Some(b'`') = lexer.peek()
                        && let Some(b'`') = lexer.peek_ahead(2)
                    {
                        tokens.push(MultilineCode(lang));
                        tokens.push(CodeBlock(code_block));
                        tokens.push(MultilineCode(vec![]));
                        lexer.advance();
                        lexer.advance();
                        lexer.advance();
                    } else {
                        // no terminating ```, skip
                        tokens.push(PlainText(b'`'));
                        tokens.push(PlainText(b'`'));
                        tokens.push(PlainText(b'`'));
                        for i in lang {
                            tokens.push(PlainText(i));
                        }
                        for i in code_block {
                            tokens.push(PlainText(i));
                        }
                    }
                } else {
                    tokens.push(MarkdownToken::BackTick);
                    lexer.advance();
                }
            }
            // not handling the case (yet) where two or more spaces are treated as a new line
            Some(b'\n') | Some(b'\r') => {
                tokens.push(MarkdownToken::Newline);
                lexer.advance();
            }
            Some(b'>') => {
                lexer.advance();
                let text = lexer.read_line();
                tokens.push(MarkdownToken::BlockQuote(text));
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
                if let Some(b' ') = lexer.peek()
                    && (Some(&MarkdownToken::Tab) == tokens.last()
                        || Some(&MarkdownToken::Newline) == tokens.last())
                {
                    tokens.push(MarkdownToken::Dash);
                    lexer.advance(); // move one for ' '
                } else {
                    tokens.push(PlainText(b'-'));
                }
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
                if lexer.peek() == Some(b'.') || lexer.peek() == Some(b')') {
                    tokens.push(MarkdownToken::Number(i));
                    lexer.advance();
                    lexer.advance();
                } else {
                    tokens.push(PlainText(i));
                    lexer.advance();
                }
            }
            Some(x) => {
                tokens.push(MarkdownToken::PlainText(x));
                lexer.advance();
            }
        }
    }
    MarkdownInformation {
        tokens,
        front_matter: None,
    }
}

#[cfg(test)]
mod tokenize_tests {
    use super::*;
    use crate::token::MarkdownToken::{
        BackTick, CurveBracketClose, CurveBracketOpen, Dash, Newline, Number, PlainText, Tab,
    };

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
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 6);
        assert_eq!(info.tokens, expected_output);
    }
    #[test]
    fn tokenize_multiline_code() {
        let input = "```py
def foo()
```"
        .as_bytes()
        .to_vec();

        let expected_output = vec![
            MultilineCode(vec![b'p', b'y']),
            CodeBlock(vec![
                b'\n', b'd', b'e', b'f', b' ', b'f', b'o', b'o', b'(', b')', b'\n',
            ]),
            MultilineCode(vec![]),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 3);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_multiline_code_no_language_marker() {
        let input = "```
a
```"
        .as_bytes()
        .to_vec();

        let expected_output = vec![
            MultilineCode(vec![]),
            CodeBlock(vec![b'\n', b'a', b'\n']),
            MultilineCode(vec![]),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 3);
        assert_eq!(info.tokens, expected_output);
    }
    #[test]
    fn tokenize_incomplete_multiline_code() {
        let input = "```py
def("
            .as_bytes()
            .to_vec();

        let expected_output = vec![
            PlainText(b'`'),
            PlainText(b'`'),
            PlainText(b'`'),
            PlainText(b'p'),
            PlainText(b'y'),
            PlainText(b'\n'),
            PlainText(b'd'),
            PlainText(b'e'),
            PlainText(b'f'),
            PlainText(b'('),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 10);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_single_line_code() {
        let input = "`()`".as_bytes().to_vec();
        let expected_output = vec![BackTick, CurveBracketOpen, CurveBracketClose, BackTick];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 4);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_single_line_blockquote() {
        let input = ">  a q!".as_bytes().to_vec();

        let expected_output = vec![MarkdownToken::BlockQuote(vec![
            b' ', b' ', b'a', b' ', b'q', b'!',
        ])];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 1);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_multi_line_blockquote() {
        let input = ">abc
>def
"
        .as_bytes()
        .to_vec();

        let expected_output = vec![
            MarkdownToken::BlockQuote(vec![b'a', b'b', b'c']),
            MarkdownToken::BlockQuote(vec![b'd', b'e', b'f']),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 2);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_invalid_headers_as_plain_text() {
        let input = "#H 1".as_bytes().to_vec();

        let expected_output = vec![
            PlainText(b'#'),
            PlainText(b'H'),
            PlainText(b' '),
            PlainText(b'1'),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 4);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_numbered_list() {
        let input = "1. h\
        2.e"
        .as_bytes()
        .to_vec();
        let expected_output = vec![
            Number(b'1'),
            PlainText(b' '),
            PlainText(b'h'),
            Number(b'2'),
            PlainText(b'e'),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 5);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_unordered_list() {
        let input = "\n- y\n- n".as_bytes().to_vec();
        let expected_output = vec![
            Newline,
            Dash,
            PlainText(b'y'),
            Newline,
            Dash,
            PlainText(b'n'),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 6);
        assert_eq!(info.tokens, expected_output);
    }

    #[test]
    fn tokenize_unordered_list_with_child_list() {
        let input = "\n- y\n\t- n".as_bytes().to_vec();
        let expected_output = vec![
            Newline,
            Dash,
            PlainText(b'y'),
            Newline,
            Tab,
            Dash,
            PlainText(b'n'),
        ];
        let info = tokenize(input);
        assert_eq!(info.tokens.len(), 7);
        assert_eq!(info.tokens, expected_output);
    }
}
