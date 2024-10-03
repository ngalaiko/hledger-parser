use chumsky::prelude::*;

use crate::utils::{end_of_line, whitespace};

#[derive(Clone, Debug, PartialEq)]
pub struct DecimalMark(char);

pub fn decimal_mark() -> impl Parser<char, DecimalMark, Error = Simple<char>> {
    just::<_, _, Simple<char>>("decimal-mark")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(one_of(".,"))
        .then_ignore(end_of_line())
        .map(DecimalMark)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_trailing() {
        let result = decimal_mark().then_ignore(end()).parse("decimal-mark , ");
        assert_eq!(result, Ok(DecimalMark(',')));
    }

    #[test]
    fn ok_comma() {
        let result = decimal_mark().then_ignore(end()).parse("decimal-mark ,");
        assert_eq!(result, Ok(DecimalMark(',')));
    }

    #[test]
    fn ok_dot() {
        let result = decimal_mark().then_ignore(end()).parse("decimal-mark .");
        assert_eq!(result, Ok(DecimalMark('.')));
    }

    #[test]
    fn ok_comment() {
        let result = decimal_mark()
            .then_ignore(end())
            .parse("decimal-mark .  ; test");
        assert_eq!(result, Ok(DecimalMark('.')));
    }

    #[test]
    fn err_format() {
        let result = decimal_mark().then_ignore(end()).parse("decimal-mark ");
        assert!(result.is_err());
    }
}
