use chumsky::prelude::*;

use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub enum Interval {}

// Some more complex intervals can be specified within period expressions, such as:
//
// biweekly (every two weeks)
// fortnightly
// bimonthly (every two months)
// every day|week|month|quarter|year
// every N days|weeks|months|quarters|years
//
// Weekly on a custom day:
//
// every Nth day of week (th, nd, rd, or st are all accepted after the number)
// every WEEKDAYNAME (full or three-letter english weekday name, case insensitive)
// Monthly on a custom day:
//
// every Nth day [of month] (31st day will be adjusted to each month's last day)
// every Nth WEEKDAYNAME [of month]
// Yearly on a custom month and day:
//
// every MM/DD [of year] (month number and day of month number)
// every MONTHNAME DDth [of year] (full or three-letter english month name, case insensitive, and day of month number)
// every DDth MONTHNAME [of year] (equivalent to the above)
pub fn period<'a>() -> impl Parser<'a, &'a str, Interval, extra::Full<Rich<'a, char>, State, ()>> {
    todo()
}
