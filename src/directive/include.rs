use chumsky::prelude::*;

use crate::{
    components::comment::comment,
    components::format::{format, Format},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    pub format: Option<Format>,
    pub path: std::path::PathBuf,
}

#[must_use]
pub fn include() -> impl Parser<char, Include, Error = Simple<char>> {
    just("include")
        .ignore_then(one_of(" \t").repeated().at_least(1))
        .ignore_then(format().then_ignore(just(":")).or_not())
        .then(text::newline().or(just(";").ignored()).not().repeated())
        .then_ignore(comment().ignored().or(text::newline()))
        .map(|(format, path)| Include {
            format,
            path: std::path::PathBuf::from(path.iter().collect::<String>().trim_end()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_without_format() {
        let result = include().then_ignore(end()).parse("include path\n");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("path")
            })
        );
    }

    #[test]
    fn ok_with_comment() {
        let result = include()
            .then_ignore(end())
            .parse("include path ; with a comment !\n");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("path")
            })
        );
    }

    #[test]
    fn ok_with_spaces() {
        let result = include()
            .then_ignore(end())
            .parse("include Path with space.csv\n");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("Path with space.csv")
            })
        );
    }

    #[test]
    fn ok_with_format() {
        let result = include().then_ignore(end()).parse("include rules:path\n");
        assert_eq!(
            result,
            Ok(Include {
                format: Some(Format::Rules),
                path: std::path::PathBuf::from("path")
            })
        );
    }

    #[test]
    fn ok_trailing() {
        let result = include().then_ignore(end()).parse("include path   \n");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("path")
            })
        );
    }

    #[test]
    fn err() {
        let result = include().then_ignore(end()).parse("inlude path");
        assert!(result.is_err());
    }
}
