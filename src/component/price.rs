use chumsky::prelude::*;

use crate::component::amount::{amount, Amount};
use crate::component::whitespace::whitespace;
use crate::state::State;

#[derive(Clone, Debug, PartialEq)]
pub enum Price {
    Unit(Amount),
    Total(Amount),
}

pub fn price<'a>() -> impl Parser<'a, &'a str, Price, extra::Full<Rich<'a, char>, State, ()>> {
    just("@")
        .repeated()
        .at_least(1)
        .at_most(2)
        .collect::<Vec<_>>()
        .then_ignore(whitespace().repeated())
        .then(amount())
        .map(|(price_type, price)| {
            if price_type.len() == 1 {
                Price::Unit(price)
            } else {
                Price::Total(price)
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::component::commodity::Commodity;

    use super::*;

    #[test]
    fn total_price() {
        let result = price().then_ignore(end()).parse("@@   $1.35").into_result();
        assert_eq!(
            result,
            Ok(Price::Total(Amount {
                is_negative: false,
                commodity: Commodity::from_str("$"),
                quantity: crate::component::quantity::Quantity {
                    mantissa: 135,
                    places: 2,
                }
            }))
        );
    }

    #[test]
    fn unit_price() {
        let result = price().then_ignore(end()).parse("@   $1.35").into_result();
        assert_eq!(
            result,
            Ok(Price::Unit(Amount {
                is_negative: false,
                commodity: Commodity::from_str("$"),
                quantity: crate::component::quantity::Quantity {
                    mantissa: 135,
                    places: 2,
                }
            }))
        );
    }
}
