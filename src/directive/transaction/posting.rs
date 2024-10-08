use chumsky::prelude::*;

mod assertion;

use super::status::{status, Status};
use crate::component::account_name::{account_name, AccountName};
use crate::component::amount::{amount, Amount};
use crate::component::price::{price, Price};
use crate::utils::{end_of_line, whitespace};

use self::assertion::{assertion, Assertion};

#[derive(Clone, Debug, PartialEq)]
pub struct Posting {
    pub status: Option<Status>,
    pub account_name: AccountName,
    pub amount: Option<Amount>,
    pub price: Option<Price>,
    pub assertion: Option<Assertion>,
}

#[must_use]
pub fn posting() -> impl Parser<char, Posting, Error = Simple<char>> {
    let posting_amount = whitespace()
        .repeated()
        .at_least(2)
        .ignore_then(amount(&crate::component::amount::Options::default()));
    let posting_price = whitespace().repeated().ignore_then(price());
    let posting_assertion = whitespace().repeated().ignore_then(assertion());
    whitespace()
        .repeated()
        .at_least(1)
        .ignore_then(status().then_ignore(whitespace()).or_not())
        .then(account_name())
        .then(posting_amount.or_not())
        .then(posting_price.or_not())
        .then(posting_assertion.or_not())
        .then_ignore(end_of_line())
        .map(
            |((((status, account_name), amount), price), assertion)| Posting {
                status,
                account_name,
                amount,
                price,
                assertion,
            },
        )
}

#[cfg(test)]
mod tests {
    use crate::component::{commodity::Commodity, quantity::Quantity};

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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                }),
                price: None,
                assertion: None,
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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: None,
                price: None,
                assertion: None,
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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                }),
                price: None,
                assertion: None,
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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: None,
                price: None,
                assertion: None,
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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: None,
                price: None,
                assertion: None,
            })
        );
    }

    #[test]
    fn with_price_assertion() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking  1 EUR@@1 USD=1 USD");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("EUR"),
                }),
                price: Some(Price::Total(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("USD"),
                })),
                assertion: Some(Assertion {
                    amount: Amount {
                        is_negative: false,
                        quantity: Quantity::from_u64(1),
                        commodity: Commodity::from_str("USD"),
                    },
                    is_subaccount_inclusive: false,
                    is_strict: false,
                }),
            })
        );
    }

    #[test]
    fn with_assertion() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking  1 USD == 1 USD");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("USD"),
                }),
                price: None,
                assertion: Some(Assertion {
                    amount: Amount {
                        is_negative: false,
                        quantity: Quantity::from_u64(1),
                        commodity: Commodity::from_str("USD"),
                    },
                    is_subaccount_inclusive: false,
                    is_strict: true,
                }),
            })
        );
    }

    #[test]
    fn with_price() {
        let result = posting()
            .then_ignore(end())
            .parse(" assets:bank:checking  1 USD @ 1 EUR");
        assert_eq!(
            result,
            Ok(Posting {
                status: None,
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking"),
                ]),
                amount: Some(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("USD"),
                }),
                price: Some(Price::Unit(Amount {
                    is_negative: false,
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("EUR"),
                })),
                assertion: None,
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
                account_name: AccountName::from_strs(&[
                    String::from("assets"),
                    String::from("bank"),
                    String::from("checking $1"),
                ]),
                amount: None,
                price: None,
                assertion: None,
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
