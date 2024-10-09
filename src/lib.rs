use std::ops::Range;

use chumsky::prelude::*;

use self::directive::{directives, Directive};

mod component;
mod directive;
mod utils;

#[allow(clippy::missing_errors_doc)]
pub fn parse(contents: &str) -> Result<Vec<Directive>, Vec<Simple<char, Range<usize>>>> {
    directives().then_ignore(end()).parse(contents)
}
