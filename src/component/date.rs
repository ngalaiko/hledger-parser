use chumsky::{
    error::Simple,
    prelude::{filter, just},
    Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    pub year: Option<u16>,
    pub month: u16,
    pub day: u16,
}

#[allow(clippy::module_name_repetitions)]
pub fn simple_date() -> impl Parser<char, Date, Error = Simple<char>> {
    let digit = filter(move |c: &char| c.is_ascii_digit());
    let year = digit
        .repeated()
        .exactly(4)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap());
    let month = digit
        .repeated()
        .at_least(1)
        .at_most(2)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap())
        .validate(|s, span, emit| {
            if !(1..=12).contains(&s) {
                emit(Simple::custom(
                    span,
                    format!("{s} must be between 1 and 12."),
                ));
            }
            s
        });
    let day = digit
        .repeated()
        .at_least(1)
        .at_most(2)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap())
        .validate(|s, span, emit| {
            if !(1..=31).contains(&s) {
                emit(Simple::custom(
                    span,
                    format!("{s} must be between 1 and 31."),
                ));
            }
            s
        });
    let with_year = |separator: char| {
        year.then_ignore(just(separator))
            .then(month)
            .then_ignore(just(separator))
            .then(day)
            .map(|((year, month), day)| Date {
                year: Some(year),
                month,
                day,
            })
    };
    let with_year = with_year('/').or(with_year('.')).or(with_year('-'));
    let without_year = |separator: char| {
        month
            .then_ignore(just(separator))
            .then(day)
            .map(|(month, day)| Date {
                year: None,
                month,
                day,
            })
    };
    let without_year = without_year('/')
        .or(without_year('.'))
        .or(without_year('-'));
    with_year.or(without_year)
}

#[cfg(test)]
mod tests {
    use chumsky::prelude::end;

    use super::*;

    #[test]
    fn simple() {
        for (input, expected) in [
            (
                "2010-01-31",
                Date {
                    year: Some(2010),
                    month: 1,
                    day: 31,
                },
            ),
            (
                "2010-01-31",
                Date {
                    year: Some(2010),
                    month: 1,
                    day: 31,
                },
            ),
            (
                "2010/01/31",
                Date {
                    year: Some(2010),
                    month: 1,
                    day: 31,
                },
            ),
            (
                "01/31",
                Date {
                    year: None,
                    month: 1,
                    day: 31,
                },
            ),
            (
                "1-31",
                Date {
                    year: None,
                    month: 1,
                    day: 31,
                },
            ),
        ] {
            let result = simple_date().then_ignore(end()).parse(input);
            assert_eq!(result, Ok(expected), "{input}");
        }
    }
}
