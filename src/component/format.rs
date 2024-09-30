use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Format {
    Journal,
    Timeclock,
    Timedot,
    Csv,
    Ssv,
    Tsv,
    Rules,
}

pub fn format() -> impl Parser<char, Format, Error = Simple<char>> {
    let journal = just("journal").map(|_| Format::Journal);
    let timeclock = just("timeclock").map(|_| Format::Timeclock);
    let timedot = just("timedot").map(|_| Format::Timedot);
    let comma_sv = just("csv").map(|_| Format::Csv);
    let semicolon_sv = just("ssv").map(|_| Format::Ssv);
    let tab_sv = just("tsv").map(|_| Format::Tsv);
    let rules = just("rules").map(|_| Format::Rules);
    journal
        .or(timeclock)
        .or(timedot)
        .or(comma_sv)
        .or(semicolon_sv)
        .or(tab_sv)
        .or(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_journal() {
        let result = format().then_ignore(end()).parse("journal");
        assert_eq!(result, Ok(Format::Journal));
    }

    #[test]
    fn ok_timeclock() {
        let result = format().then_ignore(end()).parse("timeclock");
        assert_eq!(result, Ok(Format::Timeclock));
    }

    #[test]
    fn ok_timedot() {
        let result = format().then_ignore(end()).parse("timedot");
        assert_eq!(result, Ok(Format::Timedot));
    }

    #[test]
    fn ok_csv() {
        let result = format().then_ignore(end()).parse("csv");
        assert_eq!(result, Ok(Format::Csv));
    }

    #[test]
    fn ok_ssv() {
        let result = format().then_ignore(end()).parse("ssv");
        assert_eq!(result, Ok(Format::Ssv));
    }

    #[test]
    fn ok_tsv() {
        let result = format().then_ignore(end()).parse("tsv");
        assert_eq!(result, Ok(Format::Tsv));
    }

    #[test]
    fn ok_rules() {
        let result = format().then_ignore(end()).parse("rules");
        assert_eq!(result, Ok(Format::Rules));
    }

    #[test]
    fn err() {
        let result = format().then_ignore(end()).parse("err");
        assert!(result.is_err());
    }
}
