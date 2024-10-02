use chumsky::prelude::*;

use crate::utils::{end_of_line, whitespace};

#[derive(Clone, Debug, PartialEq)]
pub struct Payee(String);

#[must_use]
pub fn payee() -> impl Parser<char, Payee, Error = Simple<char>> {
    just::<_, _, Simple<char>>("payee")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(
            text::newline()
                .or(just(";").ignored())
                .not()
                .repeated()
                .at_least(1),
        )
        .then_ignore(end_of_line())
        .collect::<String>()
        .map(|payee| Payee(payee.trim_end().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = payee().then_ignore(end()).parse("payee Test");
        assert_eq!(result, Ok(Payee("Test".to_string())));
    }

    #[test]
    fn ok_with_comment() {
        let result = payee().then_ignore(end()).parse("payee Test ; comment");
        assert_eq!(result, Ok(Payee("Test".to_string())));
    }

    #[test]
    fn ok_with_space() {
        let result = payee().then_ignore(end()).parse("payee Testing things");
        assert_eq!(result, Ok(Payee("Testing things".to_string())));
    }

    #[test]
    fn ok_with_trailing() {
        let result = payee().then_ignore(end()).parse("payee 123  ");
        assert_eq!(result, Ok(Payee("123".to_string())));
    }

    #[test]
    fn err() {
        let result = payee().then_ignore(end()).parse("paye Test");
        assert!(result.is_err());
    }
}
