use chumsky::prelude::*;

use crate::component::whitespace::whitespace;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct Tag(String);

pub fn tag() -> impl Parser<char, Tag, Error = Simple<char>> {
    just::<_, _, Simple<char>>("tag")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(
            text::newline()
                .or(just(";").ignored())
                .or(whitespace().ignored())
                .not()
                .repeated()
                .at_least(1),
        )
        .then_ignore(end_of_line())
        .collect::<String>()
        .map(|tag| Tag(tag.trim_end().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = tag().then_ignore(end()).parse("tag test-tag");
        assert_eq!(result, Ok(Tag("test-tag".to_string())));
    }

    #[test]
    fn ok_with_comment() {
        let result = tag().then_ignore(end()).parse("tag Test ; comment");
        assert_eq!(result, Ok(Tag("Test".to_string())));
    }

    #[test]
    fn err_with_space() {
        let result = tag().then_ignore(end()).parse("tag Testing things");
        assert!(result.is_err());
    }

    #[test]
    fn ok_with_trailing() {
        let result = tag().then_ignore(end()).parse("tag 123  ");
        assert_eq!(result, Ok(Tag("123".to_string())));
    }

    #[test]
    fn err() {
        let result = tag().then_ignore(end()).parse("t Test");
        assert!(result.is_err());
    }
}
