mod query;

use chumsky::prelude::*;

use crate::component::account_name::{account_name, AccountName};
use crate::component::amount::{amount, Amount};
use crate::component::comment::inline;
use crate::component::whitespace::whitespace;
use crate::directive::auto_postings::query::{query, Query};
use crate::state::State;
use crate::utils::end_of_line;

#[derive(Clone, Debug, PartialEq)]
pub struct AutoPostings {
    pub query: Query,
    pub postings: Vec<AutoPosting>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AutoPosting {
    pub account_name: AccountName,
    pub is_virtual: bool,
    pub amount: Amount,
    pub is_mul: bool,
}

pub fn auto_postings<'a>(
) -> impl Parser<'a, &'a str, AutoPostings, extra::Full<Rich<'a, char>, State, ()>> {
    let header = just("=")
        .ignore_then(whitespace().repeated())
        .ignore_then(query().then_ignore(end_of_line()))
        .then_ignore(text::newline());

    let account_name = account_name()
        .delimited_by(just('('), just(')'))
        .map(|name| (name, true))
        .or(account_name().map(|name| (name, false)));
    let posting = whitespace()
        .repeated()
        .at_least(1)
        .ignore_then(account_name)
        .then_ignore(whitespace().repeated().at_least(2))
        .then(just("*").or_not())
        .then(amount())
        .then_ignore(end_of_line())
        .map(
            |(((account_name, is_virtual), is_mul), amount)| AutoPosting {
                account_name,
                is_virtual,
                amount,
                is_mul: is_mul.is_some(),
            },
        );

    header
        .then_ignore(
            text::whitespace()
                .at_least(1)
                .then(inline())
                .then_ignore(text::newline())
                .or_not(),
        )
        .then(
            posting
                .separated_by(text::newline())
                .at_least(2)
                .collect::<Vec<_>>(),
        )
        .map(|(query, postings)| AutoPostings { query, postings })
}

#[cfg(test)]
mod tests {
    use crate::component::{commodity::Commodity, quantity::Quantity};

    use self::tests::query::Term;

    use super::*;

    #[test]
    fn full() {
        let result = auto_postings()
            .then_ignore(end())
            .parse(
                "= expenses:gifts
    assets:checking:gifts  *-1$
    (assets:checking)         1",
            )
            .into_result();
        assert_eq!(
            result,
            Ok(AutoPostings {
                query: Query {
                    terms: vec![Term {
                        r#type: None,
                        is_not: false,
                        value: String::from("expenses:gifts"),
                    }],
                },
                postings: vec![
                    AutoPosting {
                        account_name: AccountName(vec![
                            String::from("assets"),
                            String::from("checking"),
                            String::from("gifts")
                        ]),
                        is_virtual: false,
                        is_mul: true,
                        amount: Amount {
                            is_negative: true,
                            quantity: Quantity::from_u64(1),
                            commodity: Commodity::from_str("$"),
                        },
                    },
                    AutoPosting {
                        account_name: AccountName(vec![
                            String::from("assets"),
                            String::from("checking"),
                        ]),
                        is_virtual: true,
                        is_mul: false,
                        amount: Amount {
                            is_negative: false,
                            quantity: Quantity::from_u64(1),
                            commodity: Commodity::from_str(""),
                        },
                    }
                ],
            })
        );
    }
}
