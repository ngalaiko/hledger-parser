use chumsky::prelude::*;

mod posting;
mod status;

use self::posting::{posting, Posting};
use self::status::{status, Status};

use crate::{
    component::date::{simple_date, Date},
    utils::{end_of_line, whitespace},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub date: Date,
    pub status: Option<Status>,
    pub code: Option<String>,
    pub payee: String,
    pub description: Option<String>,
    pub postings: Vec<Posting>,
}

#[must_use]
pub fn transaction() -> impl Parser<char, Transaction, Error = Simple<char>> {
    let code = text::newline()
        .or(just(")").ignored()) // forbidden, because it indicates end of the code
        .not()
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just('('), just(')'));

    let payee = text::newline()
        .or(just("|").ignored()) // forbidden, because it is a description separator
        .or(just(";").ignored()) // forbidden, because it indicates comment
        .not()
        .repeated()
        .collect::<String>();

    let description = just("|").ignore_then(whitespace().repeated()).ignore_then(
        text::newline::<_, Simple<char>>()
            .or(just(";").ignored()) // forbidden, because it indicates comment
            .not()
            .repeated()
            .collect::<String>(),
    );

    let header = simple_date()
        .then(whitespace().repeated().ignore_then(status()).or_not())
        .then(whitespace().repeated().ignore_then(code).or_not())
        .then(whitespace().repeated().ignore_then(payee))
        .then(whitespace().repeated().ignore_then(description).or_not())
        .then_ignore(end_of_line());

    header
        .then_ignore(text::newline())
        .then(posting().separated_by(text::newline()).at_least(2))
        .map(|a| {
            dbg!(&a);
            a
        })
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
        let result = transaction().then_ignore(end()).parse(
            "2008/01/01 * (123) salary | january
    assets:bank:checking   $1
    income:salary  ",
        );
        assert_eq!(
            result,
            Ok(Transaction {
                date: Date {
                    year: Some(2008),
                    month: 1,
                    day: 1
                },
                code: Some(String::from("123")),
                status: Some(Status::Cleared),
                payee: String::from("salary"),
                description: Some(String::from("january")),
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
        let result = transaction().then_ignore(end()).parse(
            "2008/01/01 salary
    assets:bank:checking   $1
    income:salary  ",
        );
        assert_eq!(
            result,
            Ok(Transaction {
                date: Date {
                    year: Some(2008),
                    month: 1,
                    day: 1
                },
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
