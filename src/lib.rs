use chumsky::prelude::*;

use self::directive::{directives, Directive};

mod component;
mod directive;
mod utils;

#[allow(clippy::missing_errors_doc)]
pub fn parse(contents: &str) -> Result<Vec<Directive>, Vec<Rich<char, SimpleSpan>>> {
    directives()
        .then_ignore(end())
        .parse(contents)
        .into_result()
}
