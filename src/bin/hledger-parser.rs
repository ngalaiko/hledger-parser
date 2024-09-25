use chumsky::prelude::*;

use hledger_parser::parser;

fn main() {
    let src = std::env::args().nth(1).unwrap();

    let (tokens, errs) = parser().parse_recovery(src.as_str());
    println!("{tokens:?}");
    println!("{errs:?}");
}
