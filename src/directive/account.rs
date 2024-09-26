use chumsky::prelude::*;

use crate::components::{
    account_name::{account_name, AccountName},
    comment::comment,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub account_name: AccountName,
    // TODO: parse tags
}

pub fn account() -> impl Parser<char, Account, Error = Simple<char>> {
    just("account")
        .ignore_then(one_of(" \t").repeated().at_least(1))
        .ignore_then(account_name())
        .then_ignore(
            // The two-space requirement for same-line account comments is because ; is allowed in account names.
            just("  ").ignore_then(comment().ignored()).or(one_of(" \t")
                .repeated()
                .ignored()
                .then_ignore(text::newline())),
        )
        .map(|account_name| Account { account_name })
}

#[cfg(test)]
mod tests {
    use crate::components::account_name_part::AccountNamePart;

    use super::*;

    #[test]
    fn ok_simple() {
        let result = account()
            .then_ignore(end())
            .parse("account one:two:three\n");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_parts(&[
                    AccountNamePart::from_str("one"),
                    AccountNamePart::from_str("two"),
                    AccountNamePart::from_str("three"),
                ])
            })
        );
    }

    #[test]
    fn ok_with_padding() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three   \n");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_parts(&[
                    AccountNamePart::from_str("one"),
                    AccountNamePart::from_str("two"),
                    AccountNamePart::from_str("three"),
                ])
            })
        );
    }

    #[test]
    fn ok_comment_merged() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three ; comment \n");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_parts(&[
                    AccountNamePart::from_str("one"),
                    AccountNamePart::from_str("two"),
                    AccountNamePart::from_str("three ; comment"),
                ])
            })
        );
    }

    #[test]
    fn ok_with_comment() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three   ; comment \n");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_parts(&[
                    AccountNamePart::from_str("one"),
                    AccountNamePart::from_str("two"),
                    AccountNamePart::from_str("three"),
                ])
            })
        );
    }

    #[test]
    fn err() {
        let result = account()
            .then_ignore(end())
            .parse("acount     one:two:three   ; comment \n");
        assert!(result.is_err());
    }
}
