use chumsky::prelude::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Commodity(String);

impl Commodity {
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

pub fn commodity() -> impl Parser<char, Commodity, Error = Simple<char>> {
    let letter = filter(|c: &char| c.is_alphabetic());
    let digit = filter(|c: &char| c.is_ascii_digit());
    let space = one_of(" \t\u{a0}");
    let symbol = one_of::<_, _, Simple<char>>("$¢€£ƒ₣₧₱₨₹₽₺¥");

    let symbol = symbol.repeated().exactly(1).collect().map(Commodity);
    let simple = letter.repeated().collect().map(Commodity);
    let quoted = letter
        .or(digit)
        .or(space)
        .repeated()
        .padded_by(just("\""))
        .collect()
        .map(Commodity);

    symbol.or(quoted).or(simple)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn symbol() {
        let result = commodity().then_ignore(end()).parse("$");
        assert_eq!(result, Ok(Commodity(String::from("$"))));
    }

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
