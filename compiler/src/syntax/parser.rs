extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use chumsky::{input::Input, prelude::*};

use crate::syntax::{
    Span,
    error::{ParseError, ParseErrorKind},
    token::{Token, TokenKind},
    tree::{Binder, TypeExpr},
};

impl chumsky::span::Span for Span {
    type Offset = usize;
    type Context = usize; // file index

    fn new(context: Self::Context, range: core::ops::Range<Self::Offset>) -> Self {
        Span {
            file: context,
            start: range.start,
            end: range.end,
        }
    }

    fn context(&self) -> Self::Context {
        self.file
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

type ParserInput<'a> = chumsky::input::MappedInput<'a, Token<'a>, Span, &'a [(Token<'a>, Span)]>;

type ParserExtra<'a> = extra::Err<Rich<'a, Token<'a>, Span>>;

fn just_token<'a>(
    kind: TokenKind,
) -> impl Parser<'a, ParserInput<'a>, Token<'a>, ParserExtra<'a>> + Clone {
    any().filter(move |t: &Token| t.kind == kind)
}

fn lexeme_to_string(lexeme: &[u8]) -> String {
    String::from_utf8_lossy(lexeme).into_owned()
}

pub fn parse<'a>(
    tokens: &'a [(Token<'a>, Span)],
    eoi_span: Span,
) -> (Option<TypeExpr>, Vec<ParseError>) {
    let input = tokens.split_token_span(eoi_span);

    let parser = type_expr();

    let (output, errors) = parser.parse(input).into_output_errors();
    let errors = errors.into_iter().map(rich_to_parse_error).collect();

    (output, errors)
}


fn type_expr<'a>() -> impl Parser<'a, ParserInput<'a>, TypeExpr, ParserExtra<'a>> {
    recursive(|type_expr| {
        let atom = type_atom(type_expr.clone());

        let app = atom.clone().foldl(atom.clone().repeated(), |lhs, rhs| {
            TypeExpr::App(Box::new(lhs), Box::new(rhs))
        });

        let explicit_binder = just_token(TokenKind::LParen)
            .ignore_then(just_token(TokenKind::LowerIdentifier))
            .then_ignore(just_token(TokenKind::Colon))
            .then(type_expr.clone())
            .then_ignore(just_token(TokenKind::RParen))
            .map(|(name, ty)| Binder::Explicit(lexeme_to_string(name.lexeme), Box::new(ty)));

        let implicit_binder = just_token(TokenKind::LBrace)
            .ignore_then(just_token(TokenKind::LowerIdentifier))
            .then_ignore(just_token(TokenKind::Colon))
            .then(type_expr.clone())
            .then_ignore(just_token(TokenKind::RBrace))
            .map(|(name, ty)| Binder::Implicit(lexeme_to_string(name.lexeme), Box::new(ty)));

        let instance_binder = just_token(TokenKind::LBracket)
            .ignore_then(just_token(TokenKind::LowerIdentifier))
            .then_ignore(just_token(TokenKind::Colon))
            .then(type_expr.clone())
            .then_ignore(just_token(TokenKind::RBracket))
            .map(|(name, ty)| Binder::Instance(lexeme_to_string(name.lexeme), Box::new(ty)));

        let binder = choice((explicit_binder, implicit_binder, instance_binder));

        let pi = binder
            .clone()
            .then_ignore(just_token(TokenKind::Arrow))
            .then(type_expr.clone())
            .map(|(binder, body)| TypeExpr::Pi(binder, Box::new(body)));

        let sigma = binder
            .then_ignore(just_token(TokenKind::Product))
            .then(type_expr.clone())
            .map(|(binder, body)| TypeExpr::Sigma(binder, Box::new(body)));

        let arrow_or_product = app.clone().foldl(
            choice((
                just_token(TokenKind::Arrow).to(true),
                just_token(TokenKind::Product).to(false),
            ))
            .then(app.clone())
            .repeated(),
            |lhs, (is_arrow, rhs)| {
                if is_arrow {
                    TypeExpr::Arrow(Box::new(lhs), Box::new(rhs))
                } else {
                    TypeExpr::Sigma(
                        Binder::Explicit(String::from("_"), Box::new(lhs)),
                        Box::new(rhs),
                    )
                }
            },
        );

        choice((pi, sigma, arrow_or_product))
    })
}

fn type_atom<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, TypeExpr, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, TypeExpr, ParserExtra<'a>> + Clone {
    let var =
        just_token(TokenKind::LowerIdentifier).map(|t| TypeExpr::Var(lexeme_to_string(t.lexeme)));

    let constructor = just_token(TokenKind::UpperIdentifier)
        .map(|t| TypeExpr::Constructor(lexeme_to_string(t.lexeme)));

    let number = just_token(TokenKind::Number).map(|t| {
        let s = lexeme_to_string(t.lexeme);
        let n = s.parse::<u64>().unwrap_or(0);
        TypeExpr::Nat(n)
    });

    let string = just_token(TokenKind::String).map(|t| {
        let s = lexeme_to_string(t.lexeme);
        let inner = if s.len() >= 2 { &s[1..s.len() - 1] } else { &s };
        TypeExpr::String(String::from(inner))
    });

    let tuple_or_grouped = just_token(TokenKind::LParen)
        .ignore_then(
            type_expr
                .clone()
                .separated_by(just_token(TokenKind::Comma))
                .collect::<Vec<_>>(),
        )
        .then_ignore(just_token(TokenKind::RParen))
        .map(|items| {
            if items.len() == 1 {
                items.into_iter().next().unwrap()
            } else {
                TypeExpr::Tuple(items)
            }
        });

    choice((var, constructor, number, string, tuple_or_grouped))
}

fn rich_to_parse_error(err: Rich<'_, Token<'_>, Span>) -> ParseError {
    let span = *err.span();
    let found = err.found().map(|t| t.kind);
    let expected: Vec<TokenKind> = err
        .expected()
        .filter_map(|e| match e {
            chumsky::error::RichPattern::Token(t) => Some(t.into_inner().kind),
            chumsky::error::RichPattern::Label(_) => None,
            chumsky::error::RichPattern::EndOfInput => None,
            _ => None,
        })
        .collect();

    ParseError {
        kind: if found.is_none() {
            ParseErrorKind::UnexpectedEndOfInput
        } else {
            ParseErrorKind::UnexpectedToken
        },
        span,
        expected,
        found,
    }
}
