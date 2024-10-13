use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Interval {}

#[derive(Debug, Clone, PartialEq)]
pub struct Period {
    pub interval: Interval,
    pub begin: Option<chrono::NaiveDate>,
    pub end: Option<chrono::NaiveDate>,
}

pub fn period<'a>(
) -> impl Parser<'a, &'a str, chrono::NaiveDate, extra::Full<Rich<'a, char>, Period, ()>> {
    todo()
}
