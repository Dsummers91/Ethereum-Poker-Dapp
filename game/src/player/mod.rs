#![allow(dead_code, unused_imports)]

use card::{Card};
use table::{Table};
use hand::{Hand};

#[derive(Debug)]
pub struct Player<'a, 'b: 'a> {
  pub hand: &'a Hand<'a, 'b>,
  pub chips: u64,
}

impl<'a, 'b> Player<'a, 'b> {
    pub fn new(hand: &'a mut Hand<'a, 'b>) -> Player<'a, 'b> {
       Player{hand: hand, chips: 0}
    }
}

#[cfg(test)]
  mod tests {
//  use super::*;
    #[test]
    fn should_be_true() {
      assert_eq!(2 + 2, 4);
    }
}

