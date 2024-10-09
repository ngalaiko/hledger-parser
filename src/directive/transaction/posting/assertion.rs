use chumsky::prelude::*;

use crate::component::amount::{amount, Amount, Options};
use crate::component::whitespace::whitespace;

#[derive(Clone, Debug, PartialEq)]
pub struct Assertion {
    pub is_strict: bool,
    pub is_subaccount_inclusive: bool,
    pub amount: Amount,
}

pub fn assertion() -> impl Parser<char, Assertion, Error = Simple<char>> {
    just("=")
        .repeated()
        .at_least(1)
        .at_most(2)
        .then(just("*").or_not())
        .then_ignore(whitespace().repeated())
        .then(amount(&Options::default()))
        .map(
            |((assertion_type, subaccount_inclusive), amount)| Assertion {
                is_strict: assertion_type.len() == 2,
                is_subaccount_inclusive: subaccount_inclusive.is_some(),
                amount,
            },
        )
}

#[cfg(test)]
mod tests {
    use crate::component::commodity::Commodity;

    use super::*;

    #[test]
    fn single() {
        let result = assertion().then_ignore(end()).parse("=1$");
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
            })
        );
    }

    #[test]
    fn single_inclusive() {
        let result = assertion().then_ignore(end()).parse("=*1$");
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
            })
        );
    }

    #[test]
    fn strict() {
        let result = assertion().then_ignore(end()).parse("== 1$");
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
            })
        );
    }

    #[test]
    fn strict_inclusive() {
        let result = assertion().then_ignore(end()).parse("==* 1$");
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
            })
        );
    }
}
