use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Comment(String);

#[must_use]
pub fn comment() -> impl Parser<char, Comment, Error = Simple<char>> {
    just(";")
        .ignore_then(text::newline().not().repeated())
        .then_ignore(text::newline())
        .collect::<String>()
        .map(|comment| Comment(comment.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let result = comment().then_ignore(end()).parse("; a comment\n");
        assert_eq!(result, Ok(Comment("a comment".to_string())));
    }

    #[test]
    fn err() {
        let result = comment().then_ignore(end()).parse("not a comment");
        assert!(result.is_err());
    }
}
