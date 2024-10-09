use chumsky::prelude::*;

use crate::component::amount::{amount, Amount, Options};
use crate::component::whitespace::whitespace;

#[derive(Clone, Debug, PartialEq)]
pub enum Price {
    Unit(Amount),
    Total(Amount),
}

pub fn price() -> impl Parser<char, Price, Error = Simple<char>> {
    just("@")
        .repeated()
        .at_least(1)
        .at_most(2)
        .then_ignore(whitespace().repeated())
        .then(amount(&Options::default()))
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
        let result = price().then_ignore(end()).parse("@@   $1.35");
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
        let result = price().then_ignore(end()).parse("@   $1.35");
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
