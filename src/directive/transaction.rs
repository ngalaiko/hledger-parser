use chumsky::prelude::*;

mod posting;
mod status;

use crate::component::date::date;
use crate::component::whitespace::whitespace;
use crate::directive::transaction::posting::{posting, Posting};
use crate::directive::transaction::status::{status, Status};
use crate::state::State;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub date: chrono::NaiveDate,
    pub status: Option<Status>,
    pub code: Option<String>,
    pub payee: String,
    pub description: Option<String>,
    pub postings: Vec<Posting>,
}

pub fn transaction<'a>(
) -> impl Parser<'a, &'a str, Transaction, extra::Full<Rich<'a, char>, State, ()>> {
    let code = any()
        .and_is(text::newline().not())
        .and_is(just(")").not()) // forbidden, because it indicates end of the code
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just('('), just(')'));

    let payee = any()
        .and_is(text::newline().not())
        .and_is(just("|").not()) // forbidden, because it is a description separator
        .and_is(just(";").not()) // forbidden, because it indicates comment
        .repeated()
        .collect::<String>();

    let description = just("|").ignore_then(whitespace().repeated()).ignore_then(
        any()
            .and_is(text::newline().not())
            .and_is(just(";").not()) // forbidden, because it indicates comment
            .repeated()
            .collect::<String>(),
    );

    let header = date()
        .then(whitespace().repeated().ignore_then(status()).or_not())
        .then(whitespace().repeated().ignore_then(code).or_not())
        .then(whitespace().repeated().ignore_then(payee))
        .then(whitespace().repeated().ignore_then(description).or_not())
        .then_ignore(end_of_line());

    header
        .then_ignore(text::newline())
        .then(
            posting()
                .separated_by(text::newline())
                .at_least(2)
                .collect::<Vec<_>>(),
        )
        .map(
            |(((((date, status), code), payee), description), postings)| Transaction {
                date,
                status,
                code,
                payee: payee.trim().to_string(),
                description,
                postings,
            },
        )
}

#[cfg(test)]
mod tests {
    use crate::component::{
        account_name::AccountName, amount::Amount, commodity::Commodity, quantity::Quantity,
    };

    use super::*;

    #[test]
    fn full() {
        let result = transaction()
            .then_ignore(end())
            .parse(
                "2008/01/01 * (123) salary | january ; transaction comment
                                                 ; same comment second line
    assets:bank:checking   $1  ; posting comment
                               ; same comment second line
    income:salary  ",
            )
            .into_result();
        assert_eq!(
            result,
            Ok(Transaction {
                date: chrono::NaiveDate::from_ymd_opt(2008, 1, 1).unwrap(),
                code: Some(String::from("123")),
                status: Some(Status::Cleared),
                payee: String::from("salary"),
                description: Some(String::from("january ")),
                postings: vec![
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("assets"),
                            String::from("bank"),
                            String::from("checking"),
                        ]),
                        amount: Some(Amount {
                            is_negative: false,
                            quantity: Quantity::from_u64(1),
                            commodity: Commodity::from_str("$"),
                        }),
                        price: None,
                        assertion: None,
                    },
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("income"),
                            String::from("salary"),
                        ]),
                        amount: None,
                        price: None,
                        assertion: None,
                    }
                ],
            })
        );
    }

    #[test]
    fn simple() {
        let result = transaction()
            .then_ignore(end())
            .parse(
                "2008/01/01 salary
    assets:bank:checking   $1
    income:salary  ",
            )
            .into_result();
        assert_eq!(
            result,
            Ok(Transaction {
                date: chrono::NaiveDate::from_ymd_opt(2008, 1, 1).unwrap(),
                code: None,
                status: None,
                payee: String::from("salary"),
                description: None,
                postings: vec![
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("assets"),
                            String::from("bank"),
                            String::from("checking"),
                        ]),
                        amount: Some(Amount {
                            is_negative: false,
                            quantity: Quantity::from_u64(1),
                            commodity: Commodity::from_str("$"),
                        }),
                        price: None,
                        assertion: None,
                    },
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("income"),
                            String::from("salary"),
                        ]),
                        amount: None,
                        price: None,
                        assertion: None,
                    }
                ],
            })
        );
    }
}
