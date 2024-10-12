use chumsky::prelude::*;

use crate::component::comment::{inline, Comment};
use crate::component::whitespace::whitespace;

pub fn end_of_line<'a>() -> impl Parser<'a, &'a str, Option<Comment>, extra::Err<Rich<'a, char>>> {
    end_of_line_prefixed(0)
}

pub fn end_of_line_prefixed<'a>(
    prefix_whitespace: usize,
) -> impl Parser<'a, &'a str, Option<Comment>, extra::Err<Rich<'a, char>>> {
    whitespace()
        .repeated()
        .at_least(prefix_whitespace)
        .ignore_then(inline().map(Some))
        .or(whitespace().repeated().map(|()| None))
}
