use alloc::string::ToString;

use crate::module::{name::QualifiedName, unique::Unique};

pub const PRIM_MODULE_ID: &str = "prim";

pub fn prim_nat() -> QualifiedName {
    QualifiedName::new(Unique::new(
        0,
        PRIM_MODULE_ID.to_string(),
        Some("Nat".to_string()),
    ))
}

pub fn prim_string() -> QualifiedName {
    QualifiedName::new(Unique::new(
        1,
        PRIM_MODULE_ID.to_string(),
        Some("String".to_string()),
    ))
}

pub fn prim_type() -> QualifiedName {
    QualifiedName::new(Unique::new(
        2,
        PRIM_MODULE_ID.to_string(),
        Some("Type".to_string()),
    ))
}

pub fn prim_fin() -> QualifiedName {
    QualifiedName::new(Unique::new(
        3,
        PRIM_MODULE_ID.to_string(),
        Some("Fin".to_string()),
    ))
}