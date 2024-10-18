mod account;
mod auto_posting;
mod commodity;
mod decimal_mark;
mod include;
mod payee;
mod price;
mod tag;
mod transaction;
mod year;

use chumsky::prelude::*;

use crate::component::comment::{block, inline, line};
use crate::component::whitespace::whitespace;
use crate::directive::account::{account, Account};
use crate::directive::auto_posting::{auto_posting, AutoPosting};
use crate::directive::commodity::{commodity, Commodity};
use crate::directive::decimal_mark::{decimal_mark, DecimalMark};
use crate::directive::include::{include, Include};
use crate::directive::payee::{payee, Payee};
use crate::directive::price::{price, Price};
use crate::directive::tag::{tag, Tag};
use crate::directive::year::{year, Year};
use crate::state::State;

#[derive(Clone, Debug)]
pub enum Directive {
    Account(Account),
    AutoPosting(AutoPosting),
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
        .or(auto_posting().map(Directive::AutoPosting))
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
