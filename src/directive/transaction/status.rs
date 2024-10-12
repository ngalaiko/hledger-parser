use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    // !
    Pending,
    // *
    Cleared,
}

pub fn status<'a>() -> impl Parser<'a, &'a str, Status, extra::Err<Rich<'a, char>>> {
    choice([just("!").to(Status::Pending), just("*").to(Status::Cleared)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending() {
        let result = status().then_ignore(end()).parse("!").into_result();
        assert_eq!(result, Ok(Status::Pending));
    }

    #[test]
    fn cleared() {
        let result = status().then_ignore(end()).parse("*").into_result();
        assert_eq!(result, Ok(Status::Cleared));
    }

    #[test]
    fn error() {
        let result = status().then_ignore(end()).parse("?").into_result();
        assert!(result.is_err());
    }
}
