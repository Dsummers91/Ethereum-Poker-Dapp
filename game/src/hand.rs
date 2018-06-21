use card::{Card, Suit};
use std;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand (pub Vec<Card>);

impl Hand where Card: std::fmt::Debug {
  pub fn new(cards: Vec<Card>) -> Hand {
    Hand(cards)
  }

  pub fn values(self: &Hand) -> Vec<u8> {
    let cards = &self.0;
    let mut values: Vec<u8> = Vec::new();
    for card in cards.iter() {
        values.append(&mut card.value());
    }
    values
  }

  pub fn len(&self) -> usize {
      self.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::Suit;
  #[test]
  fn should_create_a_hand() {
    let hand: Hand = Hand::new(vec![Card{suit:Suit::Hearts, rank:2}]);
    assert_eq!(hand, hand)
  }

  #[test]
  fn should_give_values() {
    let hand: Hand = Hand::new(vec![
      Card{suit:Suit::Hearts, rank:14}, 
      Card{suit:Suit::Diamonds, rank:2}, 
      Card{suit:Suit::Hearts, rank:3}, 
      Card{suit:Suit::Hearts, rank:7}, 
      Card{suit:Suit::Hearts, rank:10}
    ]);
    assert_eq!(hand.values(), vec![1,14,2,3,7,10])
  }

  #[test]
  fn should_create_a_pair() {
    let hand: Hand = Hand::new(vec![
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Diamonds, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}
    ]);
    assert_eq!(hand, hand)
  }
}
