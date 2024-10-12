use chumsky::prelude::*;

use crate::component::whitespace::whitespace;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct Payee(String);

pub fn payee<'a>() -> impl Parser<'a, &'a str, Payee, extra::Err<Rich<'a, char>>> {
    just("payee")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(
            any()
                .and_is(text::newline().not())
                .and_is(just(";").not())
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .then_ignore(end_of_line())
        .map(|payee| Payee(payee.trim_end().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = payee().then_ignore(end()).parse("payee Test").into_result();
        assert_eq!(result, Ok(Payee("Test".to_string())));
    }

    #[test]
    fn ok_with_comment() {
        let result = payee()
            .then_ignore(end())
            .parse("payee Test ; comment")
            .into_result();
        assert_eq!(result, Ok(Payee("Test".to_string())));
    }

    #[test]
    fn ok_with_space() {
        let result = payee()
            .then_ignore(end())
            .parse("payee Testing things")
            .into_result();
        assert_eq!(result, Ok(Payee("Testing things".to_string())));
    }

    #[test]
    fn ok_with_trailing() {
        let result = payee()
            .then_ignore(end())
            .parse("payee 123  ")
            .into_result();
        assert_eq!(result, Ok(Payee("123".to_string())));
    }

    #[test]
    fn err() {
        let result = payee().then_ignore(end()).parse("paye Test").into_result();
        assert!(result.is_err());
    }
}
