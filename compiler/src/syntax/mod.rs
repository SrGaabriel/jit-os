pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod tree;

pub struct SourceFile<'a> {
    pub id: usize,
    pub name: &'a str,
    pub source: &'a [u8],
    pub package: Option<&'a str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePos {
    pub file_index: usize,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: SourcePos,
    pub end: SourcePos,
}

impl SourcePos {
    pub fn start_of_file(file_index: usize) -> Self {
        Self {
            file_index,
            line: 1,
            column: 1,
            byte_offset: 0,
        }
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.byte_offset += 1;
        self.column = 1;
    }

    pub fn row(&mut self) {
        self.column += 1;
        self.byte_offset += 1;
    }

    pub fn advance_by_char(&mut self, c: char) {
        self.column += 1;
        self.byte_offset += c.len_utf8();
    }
}

impl PartialOrd for SourcePos {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.byte_offset.cmp(&other.byte_offset))
    }
}

impl Ord for SourcePos {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.byte_offset.cmp(&other.byte_offset)
    }
}
