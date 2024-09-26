mod account;
mod decimal_mark;
mod include;
mod payee;

use chumsky::prelude::*;

use self::{
    account::{account, Account},
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
    Account(Account),
}

#[must_use]
pub fn directive() -> impl Parser<char, Directive, Error = Simple<char>> {
    include()
        .map(Directive::Include)
        .or(decimal_mark().map(Directive::DecimalMark))
        .or(payee().map(Directive::Payee))
        .or(account().map(Directive::Account))
}

#[must_use]
pub fn directives() -> impl Parser<char, Vec<Directive>, Error = Simple<char>> {
    directive().repeated()
}
