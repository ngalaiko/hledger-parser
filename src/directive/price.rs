use chumsky::prelude::*;

use crate::component::amount::{amount, Amount};
use crate::component::commodity::{commodity, Commodity};
use crate::component::date::date;
use crate::component::time::time;
use crate::component::whitespace::whitespace;
use crate::state::State;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct Price {
    pub date: chrono::NaiveDate,
    pub commodity: Commodity,
    pub amount: Amount,
}

pub fn price<'a>() -> impl Parser<'a, &'a str, Price, extra::Full<Rich<'a, char>, State, ()>> {
    just("P")
        .ignore_then(whitespace().repeated().at_least(1))
        .ignore_then(date())
        .then_ignore(whitespace().repeated().at_least(1))
        .then_ignore(time().then(whitespace().repeated().at_least(1)).or_not())
        .then(commodity())
        .then_ignore(whitespace().repeated().at_least(1))
        .then(amount())
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
        let result = price()
            .then_ignore(end())
            .parse("P 2009-01-01 € $1.35")
            .into_result();
        assert_eq!(
            result,
            Ok(Price {
                date: chrono::NaiveDate::from_ymd_opt(2009, 1, 1).unwrap(),
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
            .parse("P 2024-04-18 00:00:00 BTC 691747.70790400 SEK")
            .into_result();
        assert_eq!(
            result,
            Ok(Price {
                date: chrono::NaiveDate::from_ymd_opt(2024, 4, 18).unwrap(),
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
            .parse("P 2009-01-01 € $1.35  ; with comment")
            .into_result();
        assert_eq!(
            result,
            Ok(Price {
                date: chrono::NaiveDate::from_ymd_opt(2009, 1, 1).unwrap(),
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
