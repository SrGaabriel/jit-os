use crate::syntax::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntacticError<'a> {
    pub kind: SyntacticErrorKind,
    pub span: SourceSpan,
    pub lexeme: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntacticErrorKind {
    UnexpectedEndOfInput,
    InvalidToken,
    UnterminatedString,
    UnexpectedChar(char),
    ExpectedCharacter { expected: char, found: Option<char> },
}
