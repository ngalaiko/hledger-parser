use chumsky::prelude::*;

pub fn whitespace<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    one_of(" \t\u{a0}").ignored()
}
