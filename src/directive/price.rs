use chumsky::prelude::*;

use crate::component::whitespace::whitespace;
use crate::{
    component::{
        amount::{amount, Amount, Options},
        commodity::{commodity, Commodity},
        date::{simple_date, Date},
        time::time,
    },
    utils::end_of_line,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Price {
    pub date: Date,
    pub commodity: Commodity,
    pub amount: Amount,
}

#[must_use]
pub fn price() -> impl Parser<char, Price, Error = Simple<char>> {
    just("P")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(simple_date())
        .then_ignore(whitespace().repeated().at_least(1))
        .then_ignore(time().then(whitespace().repeated().at_least(1)).or_not())
        .then(commodity())
        .then_ignore(whitespace().repeated().at_least(1))
        .then(amount(&Options {
            quantity: crate::component::quantity::Options {
                require_decimal: false,
            },
        }))
        .then_ignore(end_of_line())
        .map(|((date, commodity), amount)| Price {
            date,
            commodity,
            amount,
        })
}

#[cfg(test)]
mod tests {
    use crate::component::quantity::Quantity;

    use super::*;

    #[test]
    fn simple() {
        let result = price().then_ignore(end()).parse("P 2009-01-01 € $1.35");
        assert_eq!(
            result,
            Ok(Price {
                date: Date {
                    year: Some(2009),
                    month: 1,
                    day: 1,
                },
                commodity: Commodity::from_str("€"),
                amount: Amount {
                    quantity: Quantity {
                        mantissa: 135,
                        places: 2
                    },
                    commodity: Commodity::from_str("$"),
                    ..Amount::default()
                },
            })
        );
    }

    #[test]
    fn with_time() {
        let result = price()
            .then_ignore(end())
            .parse("P 2024-04-18 00:00:00 BTC 691747.70790400 SEK");
        assert_eq!(
            result,
            Ok(Price {
                date: Date {
                    year: Some(2024),
                    month: 4,
                    day: 18,
                },
                commodity: Commodity::from_str("BTC"),
                amount: Amount {
                    quantity: Quantity {
                        mantissa: 69_174_770_790_400,
                        places: 8
                    },
                    commodity: Commodity::from_str("SEK"),
                    ..Amount::default()
                },
            })
        );
    }

    #[test]
    fn comment() {
        let result = price()
            .then_ignore(end())
            .parse("P 2009-01-01 € $1.35  ; with comment");
        assert_eq!(
            result,
            Ok(Price {
                date: Date {
                    year: Some(2009),
                    month: 1,
                    day: 1,
                },
                commodity: Commodity::from_str("€"),
                amount: Amount {
                    quantity: Quantity {
                        mantissa: 135,
                        places: 2
                    },
                    commodity: Commodity::from_str("$"),
                    ..Amount::default()
                },
            })
        );
    }
}
