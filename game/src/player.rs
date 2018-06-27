use card::{Card};
use table::{Table};
use hand::{Hand};

#[derive(Debug)]
pub struct Player<'a> {
  pub hand: &'a Hand<'a>,
  pub chips: u64,
}

#[cfg(test)]
  mod tests {
//  use super::*;
    #[test]
    fn should_be_true() {
      assert_eq!(2 + 2, 4);
    }
}

