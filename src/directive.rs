mod account;
mod commodity;
mod decimal_mark;
mod include;
mod payee;
mod price;
mod transaction;

use chumsky::prelude::*;

use crate::utils::whitespace;

use self::{
    account::{account, Account},
    commodity::{commodity, Commodity},
    decimal_mark::{decimal_mark, DecimalMark},
    include::{include, Include},
    payee::{payee, Payee},
    price::{price, Price},
    transaction::{transaction, Transaction},
};

pub type Span = std::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Directive {
    Account(Account),
    Commodity(Commodity),
    DecimalMark(DecimalMark),
    Include(Include),
    Payee(Payee),
    Price(Price),
    Transaction(Transaction),
}

#[must_use]
pub fn directive() -> impl Parser<char, Directive, Error = Simple<char>> {
    account()
        .map(Directive::Account)
        .or(commodity().map(Directive::Commodity))
        .or(decimal_mark().map(Directive::DecimalMark))
        .or(include().map(Directive::Include))
        .or(payee().map(Directive::Payee))
        .or(price().map(Directive::Price))
        .or(transaction().map(Directive::Transaction))
}

#[must_use]
pub fn directives() -> impl Parser<char, Vec<Directive>, Error = Simple<char>> {
    directive()
        .map(Some)
        .or(whitespace().repeated().map(|_| None))
        .separated_by(text::newline())
        .map(|directives| directives.into_iter().flatten().collect())
}
