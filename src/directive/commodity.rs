use chumsky::prelude::*;

use crate::component::whitespace::whitespace;
use crate::{
    component::{
        amount::{self, amount, Amount},
        commodity::{commodity as parse_commodity, Commodity as ParsedCommodity},
        quantity,
    },
    utils::end_of_line,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Commodity {
    Amount(Amount),
    Commodity(ParsedCommodity),
}

pub fn commodity() -> impl Parser<char, Commodity, Error = Simple<char>> {
    just("commodity")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(
            amount(&amount::Options {
                quantity: quantity::Options {
                    require_decimal: true,
                },
            })
            .map(Commodity::Amount)
            .or(parse_commodity().map(Commodity::Commodity)),
        )
        .then_ignore(end_of_line())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_symbol() {
        let result = commodity().then_ignore(end()).parse("commodity $1000.00");
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str("$"),
                quantity: quantity::Quantity {
                    mantissa: 100_000,
                    places: 2,
                },
                is_negative: false,
            }))
        );
    }

    #[test]
    fn no_symbol() {
        let result = commodity()
            .then_ignore(end())
            .parse("commodity 1,000,000.0000");
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str(""),
                quantity: quantity::Quantity {
                    mantissa: 10_000_000_000,
                    places: 4,
                },
                is_negative: false,
            }))
        );
    }

    #[test]
    fn comment() {
        let result = commodity()
            .then_ignore(end())
            .parse("commodity 1. USD ; with comment");
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str("USD"),
                quantity: quantity::Quantity {
                    mantissa: 1,
                    places: 0,
                },
                is_negative: false,
            }))
        );
    }

    #[test]
    fn just_currency() {
        let result = commodity()
            .then_ignore(end())
            .parse("commodity \"AAAA 2023\"  ");
        assert_eq!(
            result,
            Ok(Commodity::Commodity(ParsedCommodity::from_str("AAAA 2023")))
        );
    }
}
