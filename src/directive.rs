mod account;
mod commodity;
mod decimal_mark;
mod include;
mod payee;
mod price;
mod tag;
mod transaction;

use chumsky::prelude::*;

use crate::component::{
    comment::{block, inline, line},
    whitespace::whitespace,
};

use self::{
    account::{account, Account},
    commodity::{commodity, Commodity},
    decimal_mark::{decimal_mark, DecimalMark},
    include::{include, Include},
    payee::{payee, Payee},
    price::{price, Price},
    tag::{tag, Tag},
    transaction::{transaction, Transaction},
};

#[derive(Clone, Debug)]
pub enum Directive {
    Account(Account),
    Commodity(Commodity),
    DecimalMark(DecimalMark),
    Include(Include),
    Payee(Payee),
    Price(Price),
    Tag(Tag),
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
        .or(tag().map(Directive::Tag))
        .or(transaction().map(Directive::Transaction))
}

#[must_use]
pub fn directives() -> impl Parser<char, Vec<Directive>, Error = Simple<char>> {
    directive()
        .map(Some)
        .or(inline().map(|_| None))
        .or(line().map(|_| None))
        .or(block().map(|_| None))
        .or(whitespace().repeated().map(|_| None))
        .separated_by(text::newline())
        .map(|directives| directives.into_iter().flatten().collect())
}
