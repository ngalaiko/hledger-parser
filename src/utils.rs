use chumsky::prelude::*;

use crate::component::comment::comment;

pub fn whitespace() -> impl Parser<char, (), Error = Simple<char>> {
    one_of(" \t\u{a0}").ignored()
}

pub fn end_of_line() -> impl Parser<char, (), Error = Simple<char>> {
    end_of_line_prefixed(0)
}

pub fn end_of_line_prefixed(
    prefix_whitespace: usize,
) -> impl Parser<char, (), Error = Simple<char>> {
    whitespace()
        .repeated()
        .at_least(prefix_whitespace)
        .ignore_then(comment().ignored())
        .or(whitespace().repeated().ignored())
}
