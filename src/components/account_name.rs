use chumsky::prelude::*;

use super::account_name_part::{account_name_part, AccountNamePart};

#[derive(Debug, Clone, PartialEq)]
pub struct AccountName(Vec<AccountNamePart>);

impl AccountName {
    pub fn from_parts(parts: &[AccountNamePart]) -> Self {
        Self(parts.to_vec())
    }
}

pub fn account_name() -> impl Parser<char, AccountName, Error = Simple<char>> {
    account_name_part()
        .then_ignore(just(":").ignored())
        .repeated()
        .then(account_name_part())
        .map(|(parts, last_part)| {
            AccountName(
                parts
                    .into_iter()
                    .chain(std::iter::once(AccountNamePart::from_str(
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
        assert_eq!(
            result,
            Ok(AccountName(vec![AccountNamePart::from_str("account")]))
        );
    }

    #[test]
    fn ok_complex() {
        let result = account_name()
            .then_ignore(end())
            .parse("account:second level:third\"level");
        assert_eq!(
            result,
            Ok(AccountName(vec![
                AccountNamePart::from_str("account"),
                AccountNamePart::from_str("second level"),
                AccountNamePart::from_str("third\"level"),
            ]))
        );
    }
}
