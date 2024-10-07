use chumsky::prelude::*;

use crate::{
    component::account_name::{account_name, AccountName},
    utils::{end_of_line_prefixed, whitespace},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub account_name: AccountName,
}

pub fn account() -> impl Parser<char, Account, Error = Simple<char>> {
    just("account")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(account_name())
        .then_ignore(
            end_of_line_prefixed(2), // The two-space requirement for same-line account comments is because ; is allowed in account names.
        )
        .map(|account_name| Account { account_name })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_simple() {
        let result = account().then_ignore(end()).parse("account one:two:three");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_strs(&[
                    String::from("one"),
                    String::from("two"),
                    String::from("three"),
                ])
            })
        );
    }

    #[test]
    fn ok_with_padding() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three   ");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_strs(&[
                    String::from("one"),
                    String::from("two"),
                    String::from("three"),
                ])
            })
        );
    }

    #[test]
    fn ok_comment_merged() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three ; comment ");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_strs(&[
                    String::from("one"),
                    String::from("two"),
                    String::from("three ; comment"),
                ])
            })
        );
    }

    #[test]
    fn ok_with_comment() {
        let result = account()
            .then_ignore(end())
            .parse("account     one:two:three   ; comment ");
        assert_eq!(
            result,
            Ok(Account {
                account_name: AccountName::from_strs(&[
                    String::from("one"),
                    String::from("two"),
                    String::from("three"),
                ])
            })
        );
    }

    #[test]
    fn err() {
        let result = account()
            .then_ignore(end())
            .parse("acount     one:two:three   ; comment ");
        assert!(result.is_err());
    }
}
