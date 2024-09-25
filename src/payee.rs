use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Payee(String);

pub fn parser() -> impl Parser<char, Payee, Error = Simple<char>> {
    just::<_, _, Simple<char>>("payee")
        .ignore_then(just(" ").repeated())
        .ignore_then(text::newline().not().repeated())
        .then_ignore(text::newline())
        .collect::<String>()
        .map(|payee| Payee(payee.trim_end().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = parser().parse("payee Test\n");
        assert_eq!(result, Ok(Payee("Test".to_string())));
    }

    #[test]
    fn ok_with_space() {
        let result = parser().parse("payee Testing things\n");
        assert_eq!(result, Ok(Payee("Testing things".to_string())));
    }

    #[test]
    fn ok_with_trailing() {
        let result = parser().parse("payee 123  \n");
        assert_eq!(result, Ok(Payee("123".to_string())));
    }

    #[test]
    fn err() {
        let result = parser().parse("paye Test\n");
        assert!(result.is_err());
    }
}
