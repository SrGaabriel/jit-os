use alloc::string::String;

use crate::{module::unique::Unique, syntax::Span};

pub enum Name {
    Variable {
        unique: Unique,
        de_bruijn_idx: usize,
        span: Span
    },
    Intrinsic {
        unique: Unique,
        span: Span
    },
    Struct {
        unique: Unique,
        name: String,
        span: Span
    }
}

impl Name {
    pub fn unique(&self) -> &Unique {
        match self {
            Name::Variable { unique, .. } => unique,
            Name::Intrinsic { unique, .. } => unique,
            Name::Struct { unique, .. } => unique,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Name::Variable { span, .. } => span,
            Name::Intrinsic { span, .. } => span,
            Name::Struct { span, .. } => span,
        }
    }    
}