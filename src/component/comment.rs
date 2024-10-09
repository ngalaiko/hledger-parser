use chumsky::prelude::*;

use crate::component::whitespace::whitespace;

#[derive(Clone, Debug, PartialEq)]
pub struct Comment(String);

pub fn comment() -> impl Parser<char, Comment, Error = Simple<char>> {
    let comment = just(";")
        .ignore_then(text::newline().not().repeated())
        .collect::<String>();
    let prefixed_comment =
        text::newline().ignore_then(whitespace().repeated().at_least(1).ignore_then(comment));
    comment
        .then(prefixed_comment.repeated())
        .map(|(first, rest)| {
            Comment(
                std::iter::once(first)
                    .chain(rest)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let result = comment().then_ignore(end()).parse("; a comment");
        assert_eq!(result, Ok(Comment(" a comment".to_string())));
    }

    #[test]
    fn multiline() {
        let result = comment()
            .then_ignore(end())
            .parse("; a comment\n ; continuation");
        assert_eq!(result, Ok(Comment(" a comment\n continuation".to_string())));
    }

    #[test]
    fn err() {
        let result = comment().then_ignore(end()).parse("not a comment");
        assert!(result.is_err());
    }
}
