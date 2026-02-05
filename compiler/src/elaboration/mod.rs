use alloc::{boxed::Box};

use crate::{module::name::Name, syntax::tree::Literal};

pub enum Term {
    Ref(Name),
    App(Box<Term>, Box<Term>),
    Lam(Box<Term>),
    Pi(Box<Term>, Box<Term>),
    Sigma(Box<Term>, Box<Term>),
    Let(Box<Term>, Box<Term>, Box<Term>),
    Lit(Literal),
    Meta(usize),
}