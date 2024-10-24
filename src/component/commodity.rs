use chumsky::prelude::*;

use crate::state::State;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Commodity(String);

impl Commodity {
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub fn commodity<'a>() -> impl Parser<'a, &'a str, Commodity, extra::Full<Rich<'a, char>, State, ()>>
{
    let letter = any().filter(|c: &char| c.is_alphabetic());
    let symbol = one_of("$¢€£ƒ₣₧₱₨₹₽₺¥");

    let symbol = symbol.repeated().exactly(1).collect().map(Commodity);
    let simple = letter.repeated().collect().map(Commodity);
    let quoted = any()
        .and_is(just("\"").not())
        .repeated()
        .collect::<String>()
        .padded_by(just("\""))
        .map(Commodity);

    symbol.or(quoted).or(simple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn symbol() {
        let result = commodity().then_ignore(end()).parse("$").into_result();
        assert_eq!(result, Ok(Commodity(String::from("$"))));
    }

    #[test]
    pub fn simple() {
        let result = commodity().then_ignore(end()).parse("USD").into_result();
        assert_eq!(result, Ok(Commodity(String::from("USD"))));
    }

    #[test]
    pub fn quoted() {
        let result = commodity()
            .then_ignore(end())
            .parse("\"green apples\"")
            .into_result();
        assert_eq!(result, Ok(Commodity(String::from("green apples"))));
    }

    #[test]
    pub fn complex_quoted() {
        let result = commodity()
            .then_ignore(end())
            .parse("\"Hello 1 - I am a very complex commodity 😎\"")
            .into_result();
        assert_eq!(
            result,
            Ok(Commodity(String::from(
                "Hello 1 - I am a very complex commodity 😎"
            )))
        );
    }

    #[test]
    pub fn error() {
        let result = commodity().then_ignore(end()).parse("123").into_errors();
        assert!(!result.is_empty());
    }
}
