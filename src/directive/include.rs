use chumsky::prelude::*;

mod format;

use self::format::{format, Format};
use crate::component::whitespace::whitespace;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    pub format: Option<Format>,
    pub path: std::path::PathBuf,
}

#[must_use]
pub fn include() -> impl Parser<char, Include, Error = Simple<char>> {
    just("include")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(format().then_ignore(just(":")).or_not())
        .then(text::newline().or(just(";").ignored()).not().repeated())
        .then_ignore(end_of_line())
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
        let result = include().then_ignore(end()).parse("include path");
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
            .parse("include path  ; with a comment !");
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
            .parse("include Path with space.csv");
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
        let result = include().then_ignore(end()).parse("include rules:path");
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
        let result = include().then_ignore(end()).parse("include path   ");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("path")
            })
        );
    }
}
