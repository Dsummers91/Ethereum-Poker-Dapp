#![allow(dead_code, unused_imports)]

use card::{Card};
use table::{Table};
use hand::{Hand};
use deck::{Deck};

#[derive(Debug)]
pub struct Player<'a> {
  pub hand: Hand<'a>,
  pub chips: u64,
  pub active: bool,
  pub sitting_out: bool,
}

impl<'a> Player<'a> {
    pub fn new(hand: Hand<'a>) -> Player<'a> {
       Player{hand: hand, chips: 0, active: false, sitting_out: false}
    }
    
    // TODO: Only for CashGames, Capped at 100BB's
    pub fn add_chips(&mut self, chips: u64) {
        self.chips += chips
    }
}

#[cfg(test)]
  mod tests {
  use super::*;
    #[test]
    fn should_be_able_to_add_chips() {
      let mut deck: Vec<Card> = Deck::new();
      let cards = vec![];
      let hand = Hand::new(cards);
      let mut player = Player::new(hand);
      player.add_chips(1000);
      assert_eq!(player.chips, 1000);
      player.add_chips(1000);
      assert_eq!(player.chips, 2000);
    }
}

