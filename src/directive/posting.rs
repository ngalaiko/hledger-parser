use chumsky::prelude::*;

use crate::component::account_name::{account_name, AccountName};
use crate::component::amount::{amount, Amount};
use crate::component::status::{status, Status};
use crate::utils::{end_of_line, whitespace};

#[derive(Clone, Debug, PartialEq)]
pub struct Posting {
    pub status: Option<Status>,
    pub account_name: AccountName,
    pub amount: Option<Amount>,
}

#[must_use]
pub fn posting() -> impl Parser<char, Posting, Error = Simple<char>> {
    whitespace()
        .repeated()
        .at_least(1)
        .ignore_then(status().then_ignore(whitespace()).or_not())
        .then(account_name())
        .then(
            whitespace()
                .repeated()
                .at_least(2)
                .ignore_then(amount(&crate::component::amount::Options::default()))
                .or_not(),
        )
        .then_ignore(end_of_line())
        .map(|((status, account_name), amount)| Posting {
            status,
            account_name,
            amount,
        })
}

#[cfg(test)]
mod tests {
    use crate::component::{account_name::part::Part, commodity::Commodity, quantity::Quantity};

    use super::*;

    #[test]
    fn full() {
        let result = posting()
            .then_ignore(end())
            .parse(" ! assets:bank:checking   $1");
        assert_eq!(
            result,
            Ok(Posting {
                status: Some(Status::Pending),
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                })
            })
        );
    }

    #[test]
    fn no_amount() {
        let result = posting()
            .then_ignore(end())
            .parse(" ! assets:bank:checking");
        assert_eq!(
            result,
            Ok(Posting {
                status: Some(Status::Pending),
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking"),
                ]),
                amount: None,
            })
        );
    }

    #[test]
    fn no_status() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking   $1");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                })
            })
        );
    }

    #[test]
    fn with_comment() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking  ; some comment");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking"),
                ]),
                amount: None,
            })
        );
    }

    #[test]
    fn no_status_no_amount() {
        let result = posting().then_ignore(end()).parse(" assets:bank:checking");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking"),
                ]),
                amount: None,
            })
        );
    }

    #[test]
    fn not_enough_spaces() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking $1");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_parts(&[
                    Part::from_str("assets"),
                    Part::from_str("bank"),
                    Part::from_str("checking $1"),
                ]),
                amount: None,
            })
        );
    }

    #[test]
    fn no_ident() {
        let result = posting()
            .then_ignore(end())
            .parse("assets:bank:checking $1");
        assert!(result.is_err());
    }
}
