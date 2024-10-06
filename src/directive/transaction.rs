use chumsky::prelude::*;

mod posting;

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {}

#[must_use]
pub fn transaction() -> impl Parser<char, Transaction, Error = Simple<char>> {
    todo().map(|()| Transaction {})
}

#[cfg(test)]
mod tests {
    use super::*;

  //   #[test]
  //   fn simple() {
  //       let result = transaction().then_ignore(end()).parse(
  //           "2008/01/01 income
  // assets:bank:checking   $1
  // income:salary         $-1",
  //       );
  //       assert_eq!(result, Ok(Transaction {}));
  //   }
}
