mod account;
mod commodity;
mod decimal_mark;
mod include;
mod payee;
mod price;
mod tag;
pub mod transaction;
mod year;

use chumsky::prelude::*;

use crate::{
    component::{
        comment::{block, inline, line},
        whitespace::whitespace,
    },
    state::State,
};

use self::{
    account::{account, Account},
    commodity::{commodity, Commodity},
    decimal_mark::{decimal_mark, DecimalMark},
    include::{include, Include},
    payee::{payee, Payee},
    price::{price, Price},
    tag::{tag, Tag},
    year::{year, Year},
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
    Transaction(transaction::Simple),
    PeriodicTransaction(transaction::Periodic),
    Year(Year),
}

pub fn directive<'a>() -> impl Parser<'a, &'a str, Directive, extra::Full<Rich<'a, char>, State, ()>>
{
    account()
        .map(Directive::Account)
        .or(commodity().map(Directive::Commodity))
        .or(decimal_mark().map(Directive::DecimalMark))
        .or(include().map(Directive::Include))
        .or(payee().map(Directive::Payee))
        .or(price().map(Directive::Price))
        .or(tag().map(Directive::Tag))
        .or(transaction::simple().map(Directive::Transaction))
        .or(transaction::periodic().map(Directive::PeriodicTransaction))
        .or(year().map(Directive::Year))
}

pub fn directives<'a>(
) -> impl Parser<'a, &'a str, Vec<Directive>, extra::Full<Rich<'a, char>, State, ()>> {
    directive()
        .map(Some)
        .or(inline().map(|_| None))
        .or(line().map(|_| None))
        .or(block().map(|_| None))
        .or(whitespace().repeated().map(|()| None))
        .separated_by(text::newline())
        .collect::<Vec<_>>()
        .map(|directives| directives.into_iter().flatten().collect())
}
