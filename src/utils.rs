use chumsky::prelude::*;

pub fn whitespace() -> impl Parser<char, (), Error = Simple<char>> {
    one_of(" \t\u{a0}").ignored()
}
