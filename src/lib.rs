use chumsky::prelude::*;

use self::{
    directive::{directives, Directive},
    state::State,
};

mod component;
mod directive;
mod state;
mod utils;

#[allow(clippy::missing_errors_doc)]
pub fn parse(contents: &str) -> Result<Vec<Directive>, Vec<Rich<char, SimpleSpan>>> {
    directives()
        .then_ignore(end())
        .parse_with_state(contents, &mut State::default())
        .into_result()
}
