use crate::syntax::{
    SourceFile, SourcePos, SourceSpan,
    token::{LexError, LexErrorKind, Token, TokenKind},
};

pub struct Lexer<'a> {
    source_file: &'a SourceFile<'a>,
    source_pos: SourcePos,
}

impl<'a> Lexer<'a> {
    pub fn new(source_file: &'a SourceFile<'a>) -> Self {
        Self {
            source_file,
            source_pos: SourcePos::start_of_file(source_file.id),
        }
    }

    pub fn read_all(&mut self, tokens: &mut [Token<'a>]) -> usize {
        let mut index = 0;
        while let Some(Ok(token)) = self.next() {
            if index < tokens.len() {
                tokens[index] = token;
                index += 1;
            } else {
                break;
            }
        }
        index
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let source = &self.source_file.source;

        loop {
            if self.source_pos.byte_offset >= source.len() {
                return None;
            }
            let current = source[self.source_pos.byte_offset] as char;
            match current {
                ' ' | '\t' | '\r' => self.source_pos.row(),
                '\n' => self.source_pos.newline(),
                _ => break,
            }
        }

        let current = source[self.source_pos.byte_offset] as char;
        match current {
            '0'..='9' => {
                let start = self.source_pos;
                while self.source_pos.byte_offset < source.len() {
                    let c = source[self.source_pos.byte_offset] as char;
                    if c.is_ascii_digit() {
                        self.source_pos.row();
                    } else {
                        break;
                    }
                }
                let lexeme = &source[start.byte_offset..self.source_pos.byte_offset];

                Some(Ok(Token {
                    kind: TokenKind::Number,
                    lexeme,
                    span: SourceSpan {
                        start,
                        end: self.source_pos,
                    },
                }))
            }
            '"' => {
                let start = self.source_pos;
                self.source_pos.row();
                while self.source_pos.byte_offset < source.len() {
                    let c = source[self.source_pos.byte_offset] as char;
                    if c == '"' {
                        self.source_pos.row();
                        break;
                    } else {
                        self.source_pos.row();
                    }
                }
                let lexeme = &source[start.byte_offset..self.source_pos.byte_offset];

                Some(Ok(Token {
                    kind: TokenKind::String,
                    lexeme,
                    span: SourceSpan {
                        start,
                        end: self.source_pos,
                    },
                }))
            }
            u => {
                let start = self.source_pos;
                self.source_pos.row();
                let lexeme = &source[start.byte_offset..self.source_pos.byte_offset];
                Some(Err(LexError {
                    kind: LexErrorKind::UnexpectedChar(u),
                    span: SourceSpan {
                        start,
                        end: self.source_pos,
                    },
                    lexeme,
                }))
            }
        }
    }
}
