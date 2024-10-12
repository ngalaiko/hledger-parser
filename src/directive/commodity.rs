use chumsky::prelude::*;

use crate::component::amount::{amount, Amount};
use crate::component::commodity::{commodity as parse_commodity, Commodity as ParsedCommodity};
use crate::component::whitespace::whitespace;
use crate::state::State;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub enum Commodity {
    Amount(Amount),
    Commodity(ParsedCommodity),
}

pub fn commodity<'a>() -> impl Parser<'a, &'a str, Commodity, extra::Full<Rich<'a, char>, State, ()>>
{
    just("commodity")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(
            amount()
                .map(Commodity::Amount)
                .or(parse_commodity().map(Commodity::Commodity)),
        )
        .then_ignore(end_of_line())
}

#[cfg(test)]
mod tests {
    use crate::component::quantity::Quantity;

    use super::*;

    #[test]
    fn with_symbol() {
        let result = commodity()
            .then_ignore(end())
            .parse("commodity $1000.00")
            .into_result();
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str("$"),
                quantity: Quantity {
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
            .parse("commodity 1,000,000.0000")
            .into_result();
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str(""),
                quantity: Quantity {
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
            .parse("commodity 1. USD ; with comment")
            .into_result();
        assert_eq!(
            result,
            Ok(Commodity::Amount(Amount {
                commodity: ParsedCommodity::from_str("USD"),
                quantity: Quantity {
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
            .parse("commodity \"AAAA 2023\"  ")
            .into_result();
        assert_eq!(
            result,
            Ok(Commodity::Commodity(ParsedCommodity::from_str("AAAA 2023")))
        );
    }
}
