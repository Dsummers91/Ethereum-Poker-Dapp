#![allow(dead_code, unused_imports)]

use card::{Card};
use table::{Table};
use hand::{Hand};

#[derive(Debug)]
pub struct Player<'a> {
  pub hand: Hand<'a, 'a>,
  pub chips: u64,
}

impl<'a> Player<'a> {
    pub fn new(hand: Hand<'a, 'a>) -> Player<'a> {
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

