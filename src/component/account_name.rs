use chumsky::prelude::*;

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct AccountName(Vec<String>);

impl AccountName {
    pub fn from_strs(parts: &[String]) -> Self {
        Self(parts.to_vec())
    }
}

pub fn account_name<'a>(
) -> impl Parser<'a, &'a str, AccountName, extra::Full<Rich<'a, char>, State, ()>> {
    let part = any()
        .and_is(text::newline().not())
        .and_is(just(":").not()) // forbidden, because it separates account parts
        .and_is(just("  ").not()) // forbidden, because it separates inline account comment
        .repeated()
        .at_least(1)
        .collect::<String>();
    part.separated_by(just(":"))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|parts| {
            AccountName::from_strs(
                &parts
                    .iter()
                    .map(|s| s.trim())
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = account_name()
            .then_ignore(end())
            .parse("account")
            .into_result();
        assert_eq!(result, Ok(AccountName(vec![String::from("account")])));
    }

    #[test]
    fn ok_complex() {
        let result = account_name()
            .then_ignore(end())
            .parse("account:second level:third\"level")
            .into_result();
        assert_eq!(
            result,
            Ok(AccountName(vec![
                String::from("account"),
                String::from("second level"),
                String::from("third\"level"),
            ]))
        );
    }
}
