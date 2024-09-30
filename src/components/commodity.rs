use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Commodity(String);

pub fn commodity() -> impl Parser<char, Commodity, Error = Simple<char>> {
    let letter = filter(|c: &char| c.is_alphabetic());
    let digit = filter(|c: &char| c.is_ascii_digit());
    let space = one_of(" \t");

    let simple = letter.repeated().collect().map(Commodity);
    let quoted = letter
        .or(digit)
        .or(space)
        .repeated()
        .padded_by(just("\""))
        .collect()
        .map(Commodity);

    quoted.or(simple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn simple() {
        let result = commodity().then_ignore(end()).parse("USD");
        assert_eq!(result, Ok(Commodity(String::from("USD"))));
    }

    #[test]
    pub fn quoted() {
        let result = commodity().then_ignore(end()).parse("\"green apples\"");
        assert_eq!(result, Ok(Commodity(String::from("green apples"))));
    }

    #[test]
    pub fn error() {
        let result = commodity().then_ignore(end()).parse("123");
        assert!(result.is_err());
    }
}
