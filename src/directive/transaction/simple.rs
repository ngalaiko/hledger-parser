use chumsky::prelude::*;

use crate::component::date::simple::date;
use crate::component::whitespace::whitespace;
use crate::directive::transaction::posting::{posting, Posting};
use crate::directive::transaction::status::Status;
use crate::state::State;

use super::header::header;

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
    let header = date().then_ignore(whitespace().repeated()).then(header());

    header
        .then_ignore(text::newline())
        .then(
            posting()
                .separated_by(text::newline())
                .at_least(2)
                .collect::<Vec<_>>(),
        )
        .map(|((date, header), postings)| Transaction {
            date,
            status: header.status,
            code: header.code,
            payee: header.payee,
            description: header.description,
            postings,
        })
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
