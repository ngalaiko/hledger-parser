use chumsky::prelude::*;

use crate::{
    decimal_mark::{decimal_mark, DecimalMark},
    include::{include, Include},
    payee::{payee, Payee},
};

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Directive {
    Include(Include),
    DecimalMark(DecimalMark),
    Payee(Payee),
}

#[must_use]
pub fn directive() -> impl Parser<char, Directive, Error = Simple<char>> {
    include()
        .map(Directive::Include)
        .or(decimal_mark().map(Directive::DecimalMark))
        .or(payee().map(Directive::Payee))
}

#[must_use]
pub fn directives() -> impl Parser<char, Vec<Directive>, Error = Simple<char>> {
    directive().repeated()
}
