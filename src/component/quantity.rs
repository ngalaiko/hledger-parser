use chumsky::prelude::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Quantity {
    pub mantissa: u64,
    pub places: u64,
}

impl Quantity {
    pub fn from_u64(number: u64) -> Self {
        Self {
            mantissa: number,
            places: 0,
        }
    }
}

pub fn quantity<'a>() -> impl Parser<'a, &'a str, Quantity, extra::Err<Rich<'a, char>>> {
    let fraction = fraction(',').or(fraction('.')).map(|fraction| Quantity {
        mantissa: fraction.parse().unwrap(),
        places: fraction.len().try_into().unwrap(),
    });
    let thousands = thousands(',').or(thousands('.')).map(|(first, rest)| {
        if rest.len() == 1 {
            // i.e. "1,234" - it could be either a decimal, or an integer with a thousands
            // separator. we treat it as a decimal.
            Quantity {
                mantissa: (first + rest.join("").as_ref()).parse().unwrap(),
                places: 3,
            }
        } else {
            Quantity {
                mantissa: (first + rest.join("").as_ref()).parse().unwrap(),
                places: 0,
            }
        }
    });
    let digits = digits().map(|integer| Quantity {
        mantissa: integer.parse().unwrap(),
        places: 0,
    });
    thousands_and_decimals()
        .or(thousands)
        .or(decimal())
        .or(fraction)
        .or(digits)
}

fn digit<'a>() -> impl Parser<'a, &'a str, char, extra::Err<Rich<'a, char>>> {
    any().filter(|c: &char| c.is_ascii_digit())
}

fn digits<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    digit().repeated().at_least(1).collect::<String>()
}

fn one_to_three_digits<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    digit()
        .repeated()
        .at_least(1)
        .at_most(3)
        .collect::<String>()
}

fn three_digits<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    digit().repeated().exactly(3).collect::<String>()
}

fn none_or_more_digits<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    digit().repeated().collect::<String>()
}

fn fraction<'a>(mark: char) -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    just(mark).ignore_then(none_or_more_digits())
}

fn thousands_and_decimals<'a>() -> impl Parser<'a, &'a str, Quantity, extra::Err<Rich<'a, char>>> {
    thousands(',')
        .then(fraction('.'))
        .or(thousands('.').then(fraction(',')))
        .map(|(thousands, decimals)| Quantity {
            mantissa: (thousands.0 + thousands.1.join("").as_ref() + decimals.as_ref())
                .parse()
                .unwrap(),
            places: decimals.len().try_into().unwrap(),
        })
}

fn thousands<'a>(
    mark: char,
) -> impl Parser<'a, &'a str, (String, Vec<String>), extra::Err<Rich<'a, char>>> {
    one_to_three_digits().then(
        three_digits()
            .separated_by(just(mark))
            .allow_leading()
            .at_least(1)
            .collect(),
    )
}

fn decimal<'a>() -> impl Parser<'a, &'a str, Quantity, extra::Err<Rich<'a, char>>> {
    let decimal = |mark: char| digits().then(fraction(mark));
    decimal(',')
        .or(decimal('.'))
        .map(|(integer, decimals)| Quantity {
            mantissa: (integer + decimals.as_ref()).parse().unwrap(),
            places: decimals.len().try_into().unwrap(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod quantity {
        use super::*;

        #[test]
        fn integer() {
            let result = quantity().then_ignore(end()).parse("123").into_result();
            assert_eq!(
                result,
                Ok(Quantity {
                    mantissa: 123,
                    places: 0,
                })
            );
        }

        #[test]
        fn integer_trailing() {
            let result1 = quantity().then_ignore(end()).parse("123.").into_result();
            let result2 = quantity().then_ignore(end()).parse("123,").into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 123,
                    places: 0,
                })
            );
        }

        #[test]
        fn decimals_leading() {
            let result1 = quantity().then_ignore(end()).parse(".0123").into_result();
            let result2 = quantity().then_ignore(end()).parse(",0123").into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 123,
                    places: 4,
                })
            );
        }

        #[test]
        fn decimals_invalid() {
            let result = quantity().then_ignore(end()).parse("1..23").into_result();
            assert!(result.is_err());
        }

        #[test]
        fn decimals() {
            let result1 = quantity().then_ignore(end()).parse("1.23").into_result();
            let result2 = quantity().then_ignore(end()).parse("1,23").into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 123,
                    places: 2,
                })
            );
        }

        #[test]
        fn decimals_like_thousands() {
            let result1 = quantity().then_ignore(end()).parse("1.234").into_result();
            let result2 = quantity().then_ignore(end()).parse("1,234").into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 1234,
                    places: 3,
                })
            );
        }

        #[test]
        fn thousands_trailing() {
            let result1 = quantity()
                .then_ignore(end())
                .parse("12,345,678.")
                .into_result();
            let result2 = quantity()
                .then_ignore(end())
                .parse("12.345.678,")
                .into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 12_345_678,
                    places: 0,
                })
            );
        }

        #[test]
        fn thousands_invalid() {
            let result = quantity()
                .then_ignore(end())
                .parse("12.34.678")
                .into_result();
            assert!(result.is_err());
        }

        #[test]
        fn thousands() {
            let result1 = quantity()
                .then_ignore(end())
                .parse("12,345,678")
                .into_result();
            let result2 = quantity()
                .then_ignore(end())
                .parse("12.345.678")
                .into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 12_345_678,
                    places: 0,
                })
            );
        }

        #[test]
        fn thousands_and_decimals() {
            let result1 = quantity()
                .then_ignore(end())
                .parse("12,345.678")
                .into_result();
            let result2 = quantity()
                .then_ignore(end())
                .parse("12.345,678")
                .into_result();
            assert_eq!(result1, result2);
            assert_eq!(
                result2,
                Ok(Quantity {
                    mantissa: 12_345_678,
                    places: 3,
                })
            );
        }
    }
}
