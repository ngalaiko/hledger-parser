use chumsky::prelude::*;

use super::account_name_part::{account_name_part, AccountNamePart};

#[derive(Debug, Clone, PartialEq)]
pub struct AccountName(Vec<AccountNamePart>);

pub fn account_name() -> impl Parser<char, AccountName, Error = Simple<char>> {
    account_name_part().separated_by(just(":")).map(AccountName)
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
