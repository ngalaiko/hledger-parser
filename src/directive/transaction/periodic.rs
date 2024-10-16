use chumsky::prelude::*;

use crate::component::period::{period, Period};
use crate::component::whitespace::whitespace;
use crate::directive::transaction::posting::{posting, Posting};
use crate::directive::transaction::status::Status;
use crate::state::State;

use super::header::header;

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub period: Period,
    pub status: Option<Status>,
    pub code: Option<String>,
    pub payee: String,
    pub description: Option<String>,
    pub postings: Vec<Posting>,
}

pub fn transaction<'a>(
) -> impl Parser<'a, &'a str, Transaction, extra::Full<Rich<'a, char>, State, ()>> {
    let header = just("~")
        .ignore_then(whitespace().repeated())
        .ignore_then(period())
        .then_ignore(whitespace().repeated())
        .then(header());

    header
        .then_ignore(text::newline())
        .then(
            posting()
                .separated_by(text::newline())
                .at_least(2)
                .collect::<Vec<_>>(),
        )
        .map(|((period, header), postings)| Transaction {
            period,
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
        account_name::AccountName, amount::Amount, commodity::Commodity,
        period::interval::Interval, quantity::Quantity,
    };

    use super::*;

    #[test]
    fn simple() {
        let result = transaction()
            .then_ignore(end())
            .parse(
                "~ monthly from 2023-04-15 to 2023-06-16  electricity
    expenses:utilities          $400
    assets:bank:checking",
            )
            .into_result();
        assert_eq!(
            result,
            Ok(Transaction {
                period: Period {
                    interval: Some(Interval::NthMonth(1)),
                    begin: chrono::NaiveDate::from_ymd_opt(2023, 4, 15),
                    end: chrono::NaiveDate::from_ymd_opt(2023, 6, 16),
                },
                code: None,
                status: None,
                payee: String::from("electricity"),
                description: None,
                postings: vec![
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("expenses"),
                            String::from("utilities"),
                        ]),
                        amount: Some(Amount {
                            is_negative: false,
                            quantity: Quantity::from_u64(400),
                            commodity: Commodity::from_str("$"),
                        }),
                        price: None,
                        assertion: None,
                    },
                    Posting {
                        status: None,
                        account_name: AccountName::from_strs(&[
                            String::from("assets"),
                            String::from("bank"),
                            String::from("checking"),
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
