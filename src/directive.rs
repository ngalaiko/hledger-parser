mod account;
mod commodity;
mod decimal_mark;
mod include;
mod payee;
mod posting;
mod price;

use chumsky::prelude::*;

use crate::utils::whitespace;

use self::{
    account::{account, Account},
    commodity::{commodity, Commodity},
    decimal_mark::{decimal_mark, DecimalMark},
    include::{include, Include},
    payee::{payee, Payee},
    price::{price, Price},
};

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Directive {
    Include(Include),
    DecimalMark(DecimalMark),
    Payee(Payee),
    Account(Account),
    Commodity(Commodity),
    Price(Price),
}

#[must_use]
pub fn directive() -> impl Parser<char, Directive, Error = Simple<char>> {
    include()
        .map(Directive::Include)
        .or(decimal_mark().map(Directive::DecimalMark))
        .or(payee().map(Directive::Payee))
        .or(account().map(Directive::Account))
        .or(commodity().map(Directive::Commodity))
        .or(price().map(Directive::Price))
}

#[must_use]
pub fn directives() -> impl Parser<char, Vec<Directive>, Error = Simple<char>> {
    directive()
        .map(Some)
        .or(whitespace().repeated().map(|_| None))
        .separated_by(text::newline())
        .map(|directives| directives.into_iter().flatten().collect())
}
