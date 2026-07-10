#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownHeaderToken {
    Header1(Vec<u8>),
    Header2(Vec<u8>),
    Header3(Vec<u8>),
    Header4(Vec<u8>),
    Header5(Vec<u8>),
    Header6(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MarkdownToken {
    Header(MarkdownHeaderToken),
    Newline,
    Code,
    BlockQuote,
    Asterik,
    PlainText(u8),
    SquareBracketOpen,
    SquareBracketClose,
    CurveBracketOpen,
    CurveBracketClose,
    Number(u8),
    Dash,
    Tab,
}

impl Into<u8> for MarkdownToken {
    fn into(self) -> u8 {
        match self {
            MarkdownToken::Header(_) => b'#', // incorrect
            MarkdownToken::Newline => b'\n',
            MarkdownToken::Code => b'`',
            MarkdownToken::BlockQuote => b'>',
            MarkdownToken::Asterik => b'*',
            MarkdownToken::PlainText(x) => x,
            MarkdownToken::SquareBracketOpen => b'[',
            MarkdownToken::SquareBracketClose => b']',
            MarkdownToken::CurveBracketOpen => b'(',
            MarkdownToken::CurveBracketClose => b')',
            MarkdownToken::Number(x) => x,
            MarkdownToken::Dash => b'-',
            MarkdownToken::Tab => b'\t',
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PhrasingHtmlContent {
    ParagraphPlainText(Vec<u8>),
    Strong(Vec<u8>),
    Italic(Vec<u8>),
    Code(Vec<u8>),
    Link(Vec<u8>, Vec<u8>),
    Break,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HtmlToken {
    Heading1(Vec<u8>),
    Heading2(Vec<u8>),
    Heading3(Vec<u8>),
    Heading4(Vec<u8>),
    Heading5(Vec<u8>),
    Heading6(Vec<u8>),
    Pre(Vec<u8>),
    Paragraph(Vec<PhrasingHtmlContent>),
    Article(Vec<HtmlToken>), // unused for now
    BlockQuote(Vec<PhrasingHtmlContent>),
}

impl HtmlToken {
    pub fn get_paragraph_contents(&self) -> Option<Vec<PhrasingHtmlContent>> {
        match self {
            HtmlToken::Paragraph(c) => Some(c.clone()),
            _ => None,
        }
    }
}

pub struct HtmlBody {
    pub(crate) children: Vec<HtmlToken>,
}
