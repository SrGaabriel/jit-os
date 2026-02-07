use crate::module::{unique::Unique};

// todo: reconsider this
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedName {
    pub unique: Unique, 
}

impl QualifiedName {
    pub const fn new(unique: Unique) -> Self {
        Self { unique }
    }
}