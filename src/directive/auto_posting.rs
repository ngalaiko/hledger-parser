mod query;

use chumsky::prelude::*;

use crate::state::State;

#[derive(Clone, Debug, PartialEq)]
pub struct AutoPosting {}

pub fn auto_posting<'a>(
) -> impl Parser<'a, &'a str, AutoPosting, extra::Full<Rich<'a, char>, State, ()>> {
    todo()
}
