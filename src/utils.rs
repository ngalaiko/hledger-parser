use chumsky::prelude::*;

use crate::component::comment::{inline, Comment};
use crate::component::whitespace::whitespace;

pub fn end_of_line() -> impl Parser<char, Option<Comment>, Error = Simple<char>> {
    end_of_line_prefixed(0)
}

pub fn end_of_line_prefixed(
    prefix_whitespace: usize,
) -> impl Parser<char, Option<Comment>, Error = Simple<char>> {
    whitespace()
        .repeated()
        .at_least(prefix_whitespace)
        .ignore_then(inline().map(Some))
        .or(whitespace().repeated().map(|_| None))
}
