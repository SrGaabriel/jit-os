extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;

pub trait Phase {
    type ExprAnn;
    type TyAnn;
}

pub struct Parsed;
pub struct Typed;

impl Phase for Parsed {
    type ExprAnn = ();
    type TyAnn = ();
}

impl Phase for Typed {
    type ExprAnn = ();
    type TyAnn = ();
}

pub struct Expr<P: Phase> {
    pub ann: P::ExprAnn,
    pub kind: ExprKind<P>,
}

pub enum ExprKind<P: Phase> {
    Var(String),
    App(Box<Expr<P>>, Box<Expr<P>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Var(String),
    Constructor(String),
    App(Box<TypeExpr>, Box<TypeExpr>),
    Arrow(Box<TypeExpr>, Box<TypeExpr>),
    Tuple(Vec<TypeExpr>),
    List(Box<TypeExpr>),
    Pi(Binder, Box<TypeExpr>),
    Sigma(Binder, Box<TypeExpr>),
    Nat(u64),
    String(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binder {
    Explicit(String, Box<TypeExpr>),
    Implicit(String, Box<TypeExpr>),
    Instance(String, Box<TypeExpr>),
}
