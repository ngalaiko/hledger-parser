pub mod part;

use chumsky::prelude::*;

use self::part::{part, Part};

#[derive(Debug, Clone, PartialEq)]
pub struct AccountName(Vec<Part>);

impl AccountName {
    pub fn from_parts(parts: &[Part]) -> Self {
        Self(parts.to_vec())
    }
}

pub fn account_name() -> impl Parser<char, AccountName, Error = Simple<char>> {
    part()
        .then_ignore(just(":").ignored())
        .repeated()
        .then(part())
        .map(|(parts, last_part)| {
            AccountName(
                parts
                    .into_iter()
                    .chain(std::iter::once(Part::from_str(
                        last_part.to_str().trim_end(),
                    )))
                    .collect(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = account_name().then_ignore(end()).parse("account");
        assert_eq!(result, Ok(AccountName(vec![Part::from_str("account")])));
    }

    #[test]
    fn ok_complex() {
        let result = account_name()
            .then_ignore(end())
            .parse("account:second level:third\"level");
        assert_eq!(
            result,
            Ok(AccountName(vec![
                Part::from_str("account"),
                Part::from_str("second level"),
                Part::from_str("third\"level"),
            ]))
        );
    }
}
