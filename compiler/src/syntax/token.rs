use crate::syntax::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub lexeme: &'a [u8],
    pub kind: TokenKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Number,
    String,
    EndOfFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError<'a> {
    pub kind: LexErrorKind,
    pub span: SourceSpan,
    pub lexeme: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErrorKind {
    UnexpectedChar(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream<'a> {
    pub tokens: &'a [Token<'a>],
    pub position: usize,
}
