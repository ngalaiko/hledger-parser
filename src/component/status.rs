use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    // !
    Pending,
    // *
    Cleared,
}

pub fn status() -> impl Parser<char, Status, Error = Simple<char>> {
    choice([just("!").to(Status::Pending), just("*").to(Status::Cleared)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending() {
        let result = status().then_ignore(end()).parse("!");
        assert_eq!(result, Ok(Status::Pending));
    }

    #[test]
    fn cleared() {
        let result = status().then_ignore(end()).parse("*");
        assert_eq!(result, Ok(Status::Cleared));
    }

    #[test]
    fn error() {
        let result = status().then_ignore(end()).parse("?");
        assert!(result.is_err());
    }
}
