use chumsky::prelude::*;

use crate::component::whitespace::whitespace;

#[derive(Clone, Debug, PartialEq)]
pub struct Comment(String);

pub fn line() -> impl Parser<char, Comment, Error = Simple<char>> {
    just("#")
        .ignore_then(text::newline().not().repeated())
        .collect::<String>()
        .map(Comment)
}

pub fn block() -> impl Parser<char, Comment, Error = Simple<char>> {
    text::newline()
        .or(just("end comment\n").ignored())
        .not()
        .repeated()
        .collect::<String>()
        .separated_by(text::newline())
        .delimited_by(just("comment\n"), just("end comment\n"))
        .map(|lines| Comment(lines.join("\n").trim().to_string()))
}

pub fn inline() -> impl Parser<char, Comment, Error = Simple<char>> {
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
    fn ok_line() {
        let result = line().then_ignore(end()).parse("# a comment");
        assert_eq!(result, Ok(Comment(" a comment".to_string())));
    }

    #[test]
    fn ok_inline() {
        let result = inline().then_ignore(end()).parse("; a comment");
        assert_eq!(result, Ok(Comment(" a comment".to_string())));
    }

    #[test]
    fn ok_block() {
        let result = block()
            .then_ignore(end())
            .parse("comment\nmultiline\ncomment block\nend comment\n");
        assert_eq!(result, Ok(Comment("multiline\ncomment block".to_string())));
    }

    #[test]
    fn inline_multiline() {
        let result = inline()
            .then_ignore(end())
            .parse("; a comment\n ; continuation");
        assert_eq!(result, Ok(Comment(" a comment\n continuation".to_string())));
    }

    #[test]
    fn err() {
        let result = inline().then_ignore(end()).parse("not a comment");
        assert!(result.is_err());
    }
}
