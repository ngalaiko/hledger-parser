use chumsky::prelude::*;

mod decimal_mark;
mod format;
mod include;
mod payee;

use self::{decimal_mark::DecimalMark, include::Include, payee::Payee};

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Directive {
    Include(Include),
    DecimalMark(DecimalMark),
    Payee(Payee),
}

#[must_use]
pub fn parser() -> impl Parser<char, (Vec<Directive>, Span), Error = Simple<char>> {
    let directives = include::parser()
        .map(Directive::Include)
        .or(decimal_mark::parser().map(Directive::DecimalMark))
        .or(payee::parser().map(Directive::Payee))
        .repeated();

    directives
        .then_ignore(end())
        .map_with_span(|tok, span| (tok, span))
}
