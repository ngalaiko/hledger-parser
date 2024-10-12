use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    pub year: Option<u16>,
    pub month: u16,
    pub day: u16,
}

pub fn date<'a>() -> impl Parser<'a, &'a str, Date, extra::Err<Rich<'a, char>>> {
    let digit = any().filter(|c: &char| c.is_ascii_digit());
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
        .validate(|s, e, emitter| {
            if !(1..=12).contains(&s) {
                emitter.emit(Rich::custom(
                    e.span(),
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
        .validate(|s, e, emitter| {
            if !(1..=31).contains(&s) {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("{s} must be between 1 and 31."),
                ));
            }
            s
        });
    let date = |separator: char| {
        year.then_ignore(just(separator))
            .or_not()
            .then(month)
            .then_ignore(just(separator))
            .then(day)
            .map(|((year, month), day)| Date { year, month, day })
    };
    date('/').or(date('.')).or(date('-'))
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
            let result = date().then_ignore(end()).parse(input).into_result();
            assert_eq!(result, Ok(expected), "{input}");
        }
    }
}
