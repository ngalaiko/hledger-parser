use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DecimalMark(char);

pub fn parser() -> impl Parser<char, DecimalMark, Error = Simple<char>> {
    just::<_, _, Simple<char>>("decimal-mark")
        .ignore_then(just(" ").repeated())
        .ignore_then(one_of(".,"))
        .then_ignore(just(" ").repeated())
        .then_ignore(text::newline())
        .map(DecimalMark)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_trailing() {
        let result = parser().parse("decimal-mark , \n");
        assert_eq!(result, Ok(DecimalMark(',')));
    }

    #[test]
    fn ok_comma() {
        let result = parser().parse("decimal-mark ,\n");
        assert_eq!(result, Ok(DecimalMark(',')));
    }

    #[test]
    fn ok_dot() {
        let result = parser().parse("decimal-mark .\n");
        assert_eq!(result, Ok(DecimalMark('.')));
    }

    #[test]
    fn err_format() {
        let result = parser().parse("decimal-mark \n");
        assert!(result.is_err());
    }
}
