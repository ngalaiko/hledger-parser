use chumsky::prelude::*;

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    hours: u16,
    minutes: u16,
    seconds: u16,
}

pub fn time<'a>() -> impl Parser<'a, &'a str, Time, extra::Full<Rich<'a, char>, State, ()>> {
    let digit = any().filter(move |c: &char| c.is_ascii_digit());
    let hour = digit
        .repeated()
        .exactly(2)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap())
        .validate(|s, e, emitter| {
            if !(0..=23).contains(&s) {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("{s} must be between 0 and 23."),
                ));
            }
            s
        });

    let minute = digit
        .repeated()
        .exactly(2)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap())
        .validate(|s, e, emitter| {
            if !(0..=59).contains(&s) {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("{s} must be between 0 and 59."),
                ));
            }
            s
        });
    let second = digit
        .repeated()
        .exactly(2)
        .collect::<String>()
        .map(|m| m.parse::<u16>().unwrap())
        .validate(|s, e, emitter| {
            if !(0..=59).contains(&s) {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("{s} must be between 0 and 59."),
                ));
            }
            s
        });

    hour.then_ignore(just(":"))
        .then(minute)
        .then_ignore(just(":"))
        .then(second)
        .map(|((hours, minutes), seconds)| Time {
            hours,
            minutes,
            seconds,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let result = time().then_ignore(end()).parse("00:00:00").into_result();
        assert_eq!(
            result,
            Ok(Time {
                hours: 0,
                minutes: 0,
                seconds: 0
            })
        );
    }
    #[test]
    fn error() {
        let result = time().then_ignore(end()).parse("25:00:00").into_result();
        assert!(result.is_err());
    }
}
