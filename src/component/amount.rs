use chumsky::prelude::*;

use crate::utils::whitespace;

use super::{
    commodity::{commodity, Commodity},
    quantity::{quantity, Quantity},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Amount {
    pub is_negative: bool,
    pub quantity: Quantity,
    pub commodity: Commodity,
}

pub fn amount() -> impl Parser<char, Amount, Error = Simple<char>> {
    let sign_quantity_commodity = one_of("-+")
        .then_ignore(whitespace().repeated())
        .then(quantity())
        .then_ignore(whitespace().repeated())
        .then(commodity())
        .map(|((sign, quantity), commodity)| Amount {
            quantity,
            commodity,
            is_negative: sign == '-',
        });
    let quantity_sign_commodity = quantity()
        .then_ignore(whitespace().repeated())
        .then(one_of("-+"))
        .then_ignore(whitespace().repeated())
        .then(commodity())
        .map(|((quantity, sign), commodity)| Amount {
            quantity,
            commodity,
            is_negative: sign == '-',
        });
    let sign_commodity_quantity = one_of("-+")
        .then_ignore(whitespace().repeated())
        .then(commodity())
        .then_ignore(whitespace().repeated())
        .then(quantity())
        .map(|((sign, commodity), quantity)| Amount {
            quantity,
            commodity,
            is_negative: sign == '-',
        });
    let commodity_sign_quantity = commodity()
        .then_ignore(whitespace().repeated())
        .then(one_of("-+"))
        .then_ignore(whitespace().repeated())
        .then(quantity())
        .map(|((commodity, sign), quantity)| Amount {
            quantity,
            commodity,
            is_negative: sign == '-',
        });
    let quantity_commodity = quantity()
        .then_ignore(whitespace().repeated())
        .then(commodity())
        .map(|(quantity, commodity)| Amount {
            quantity,
            commodity,
            ..Amount::default()
        });
    let commodity_quantity = commodity()
        .then_ignore(whitespace().repeated())
        .then(quantity())
        .map(|(commodity, quantity)| Amount {
            quantity,
            commodity,
            ..Amount::default()
        });
    let just_quantity = quantity().map(|quantity| Amount {
        quantity,
        ..Amount::default()
    });
    sign_quantity_commodity
        .or(quantity_sign_commodity)
        .or(sign_commodity_quantity)
        .or(commodity_sign_quantity)
        .or(quantity_commodity)
        .or(commodity_quantity)
        .or(just_quantity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantity_no_commodity() {
        let result = amount().then_ignore(end()).parse("1");
        assert_eq!(
            result,
            Ok(Amount {
                quantity: Quantity::from_u64(1),
                ..Amount::default()
            })
        );
    }

    #[test]
    fn quantity_with_commodity() {
        for (input, expected) in [
            (
                "$1",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                    ..Amount::default()
                },
            ),
            (
                "4000 AAPL",
                Amount {
                    quantity: Quantity::from_u64(4000),
                    commodity: Commodity::from_str("AAPL"),
                    ..Amount::default()
                },
            ),
            (
                "3 \"green apples\"",
                Amount {
                    quantity: Quantity::from_u64(3),
                    commodity: Commodity::from_str("green apples"),
                    ..Amount::default()
                },
            ),
        ] {
            let result = amount().then_ignore(end()).parse(input);
            assert_eq!(result, Ok(expected), "{input}");
        }
    }

    #[test]
    fn signed_quantity_with_commodity() {
        for (input, expected) in [
            (
                "-$1",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                    is_negative: true,
                },
            ),
            (
                "$-1",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                    is_negative: true,
                },
            ),
            (
                "+ $1",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                    ..Amount::default()
                },
            ),
            (
                "$-      1",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("$"),
                    is_negative: true,
                },
            ),
            (
                "-1 USD",
                Amount {
                    quantity: Quantity::from_u64(1),
                    commodity: Commodity::from_str("USD"),
                    is_negative: true,
                },
            ),
        ] {
            let result = amount().then_ignore(end()).parse(input);
            assert_eq!(result, Ok(expected), "{input}");
        }
    }
}
