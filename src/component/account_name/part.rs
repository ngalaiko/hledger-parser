use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Part(String);

impl Part {
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

pub fn part() -> impl Parser<char, Part, Error = Simple<char>> {
    text::newline()
        .or(just(":").ignored()) // forbidden, because it separates account parts
        .or(just("  ").ignored()) // forbidden, because it separates inline account comment
        .not()
        .repeated()
        .collect::<String>()
        .map(Part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = part().then_ignore(end()).parse("account");
        assert_eq!(result, Ok(Part::from_str("account")));
    }

    #[test]
    fn ok_complex() {
        let result = part().then_ignore(end()).parse("with\"quotes and spaces'");
        assert_eq!(result, Ok(Part::from_str("with\"quotes and spaces'")));
    }

    #[test]
    fn err_colon() {
        let result = part().then_ignore(end()).parse("not:valid");
        assert!(result.is_err());
    }

    #[test]
    fn err_newline() {
        let result = part().then_ignore(end()).parse("not\nvalid");
        assert!(result.is_err());
    }
}
