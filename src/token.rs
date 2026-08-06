use chrono::{DateTime, Utc};

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
    BackTick,
    MultilineCode(Vec<u8>),
    CodeBlock(Vec<u8>),
    BlockQuote(Vec<u8>),
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
            MarkdownToken::Header(_) => b'#',
            MarkdownToken::Newline => b'\n',
            MarkdownToken::BackTick => b'`',
            MarkdownToken::BlockQuote(_) => b'>',
            MarkdownToken::Asterik => b'*',
            MarkdownToken::PlainText(x) => x,
            MarkdownToken::SquareBracketOpen => b'[',
            MarkdownToken::SquareBracketClose => b']',
            MarkdownToken::CurveBracketOpen => b'(',
            MarkdownToken::CurveBracketClose => b')',
            MarkdownToken::Number(x) => x,
            MarkdownToken::Dash => b'-',
            MarkdownToken::Tab => b'\t',
            MarkdownToken::MultilineCode(_) | MarkdownToken::CodeBlock(_) => todo!(),
        }
    }
}

pub struct MarkdownInformation {
    pub(crate) front_matter: Option<Vec<(String, String)>>,
    pub(crate) tokens: Vec<MarkdownToken>,
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
pub struct OrderedListItem {
    pub(crate) order: u8,
    pub(crate) content: Vec<PhrasingHtmlContent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HtmlToken {
    Heading1(Vec<u8>),
    Heading2(Vec<u8>),
    Heading3(Vec<u8>),
    Heading4(Vec<u8>),
    Heading5(Vec<u8>),
    Heading6(Vec<u8>),
    Pre(PhrasingHtmlContent),
    Break,
    Paragraph(Vec<PhrasingHtmlContent>),
    Article(Vec<HtmlToken>), // unused for now
    BlockQuote(Vec<PhrasingHtmlContent>),
    OrderedList(Vec<OrderedListItem>)
}

impl HtmlToken {
    pub fn get_paragraph_contents(&self) -> Option<Vec<PhrasingHtmlContent>> {
        match self {
            HtmlToken::Paragraph(c) => Some(c.clone()),
            _ => None,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct BlogPost {
    pub(crate) title: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) children: Vec<HtmlToken>,
}

pub struct BlogPostMeta {
    pub(crate) title: String,
    pub(crate) date: DateTime<Utc>,
    pub(crate) link: String,
}
