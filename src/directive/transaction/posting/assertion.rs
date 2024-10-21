use chumsky::prelude::*;

use crate::component::amount::{amount, Amount};
use crate::component::price::{price, Price};
use crate::component::whitespace::whitespace;
use crate::state::State;

#[derive(Clone, Debug, PartialEq)]
pub struct Assertion {
    pub is_strict: bool,
    pub is_subaccount_inclusive: bool,
    pub amount: Amount,
    pub price: Option<Price>,
}

pub fn assertion<'a>() -> impl Parser<'a, &'a str, Assertion, extra::Full<Rich<'a, char>, State, ()>>
{
    let price = whitespace().repeated().ignore_then(price());
    just("=")
        .repeated()
        .at_least(1)
        .at_most(2)
        .collect::<Vec<_>>()
        .then(just("*").or_not())
        .then_ignore(whitespace().repeated())
        .then(amount())
        .then(price.or_not())
        .map(
            |(((assertion_type, subaccount_inclusive), amount), price)| Assertion {
                is_strict: assertion_type.len() == 2,
                is_subaccount_inclusive: subaccount_inclusive.is_some(),
                amount,
                price,
            },
        )
}

#[cfg(test)]
mod tests {
    use crate::component::commodity::Commodity;

    use super::*;

    #[test]
    fn single_with_price() {
        let result = assertion()
            .then_ignore(end())
            .parse("=1$ @@ 5 USD")
            .into_result();
        assert_eq!(
            result,
            Ok(Assertion {
                is_strict: false,
                is_subaccount_inclusive: false,
                amount: Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("$"),
                    quantity: crate::component::quantity::Quantity::from_u64(1),
                },
                price: Some(Price::Total(Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("USD"),
                    quantity: crate::component::quantity::Quantity::from_u64(5),
                })),
            })
        );
    }

    #[test]
    fn single() {
        let result = assertion().then_ignore(end()).parse("=1$").into_result();
        assert_eq!(
            result,
            Ok(Assertion {
                is_strict: false,
                is_subaccount_inclusive: false,
                amount: Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("$"),
                    quantity: crate::component::quantity::Quantity::from_u64(1),
                },
                price: None,
            })
        );
    }

    #[test]
    fn single_inclusive() {
        let result = assertion().then_ignore(end()).parse("=*1$").into_result();
        assert_eq!(
            result,
            Ok(Assertion {
                is_strict: false,
                is_subaccount_inclusive: true,
                amount: Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("$"),
                    quantity: crate::component::quantity::Quantity::from_u64(1),
                },
                price: None,
            })
        );
    }

    #[test]
    fn strict() {
        let result = assertion().then_ignore(end()).parse("== 1$").into_result();
        assert_eq!(
            result,
            Ok(Assertion {
                is_strict: true,
                is_subaccount_inclusive: false,
                amount: Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("$"),
                    quantity: crate::component::quantity::Quantity::from_u64(1),
                },
                price: None,
            })
        );
    }

    #[test]
    fn strict_inclusive() {
        let result = assertion().then_ignore(end()).parse("==* 1$").into_result();
        assert_eq!(
            result,
            Ok(Assertion {
                is_strict: true,
                is_subaccount_inclusive: true,
                amount: Amount {
                    is_negative: false,
                    commodity: Commodity::from_str("$"),
                    quantity: crate::component::quantity::Quantity::from_u64(1),
                },
                price: None,
            })
        );
    }
}
