use chumsky::prelude::*;

use crate::format::{self, Format};

#[derive(Clone, Debug, PartialEq)]
pub struct Include {
    pub format: Option<Format>,
    pub path: std::path::PathBuf,
}

pub fn parser() -> impl Parser<char, Include, Error = Simple<char>> {
    just("include")
        .ignore_then(just(" ").repeated())
        .ignore_then(format::parser().then_ignore(just(":")).or_not())
        .then(filter(|c: &char| !c.is_whitespace()).repeated())
        .then_ignore(just(" ").repeated())
        .then_ignore(text::newline())
        .map(|(format, path)| Include {
            format,
            path: std::path::PathBuf::from(path.iter().collect::<String>()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_without_format() {
        let result = parser().parse("include path\n");
        assert_eq!(
            result,
            Ok(Include {
                format: None,
                path: std::path::PathBuf::from("path")
            })
        );
    }

    #[test]
    fn ok_with_format() {
        let result = parser().parse("include rules:path\n");
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
        let result = parser().parse("include path   \n");
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
        let result = parser().parse("inlude path");
        assert!(result.is_err());
    }
}
