#[derive(Debug, PartialEq)]
pub enum HeaderToken {
    Header1(Vec<u8>),
    Header2(Vec<u8>),
    Header3(Vec<u8>),
    Header4(Vec<u8>),
    Header5(Vec<u8>),
    Header6(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Header(HeaderToken),
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
    UnorderedList,
    Tab,
}

#[derive(Debug, PartialEq)]
pub enum PhrasingHtmlContent {
    ParagraphPlainText(Vec<u8>),
    Strong(Vec<u8>),
    Italic(Vec<u8>),
    Code(Vec<u8>),
    Link(Vec<u8>, Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum HtmlToken {
    Heading1(Vec<u8>),
    Heading2(Vec<u8>),
    Heading3(Vec<u8>),
    Heading4(Vec<u8>),
    Heading5(Vec<u8>),
    Heading6(Vec<u8>),
    Paragraph(Vec<PhrasingHtmlContent>),
    Article(Vec<HtmlToken>), // unused for now
}
pub struct HtmlBody {
    pub(crate) children: Vec<HtmlToken>,
}
