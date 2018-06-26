use card::{Card, Suit};
use rankings::{get_rank, Ranks, Rank};
use std::collections::HashMap;
use std;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hand {
   pub cards: Vec<Card>,
   value: Option<Rank>
}

impl Hand where Card: std::fmt::Debug {
  pub fn new(mut cards: Vec<Card>) -> Hand {
    Hand{cards, value: None}
  }

  pub fn get_value(&mut self) -> &mut Self {
    let hand = self.clone();
    if let Some(x) = get_rank(hand) {
        self.value = Some(Rank{active_cards: vec![], rank: Ranks::HighCard}); 
    }
    self
  }

  pub fn ranks(self) -> Vec<u8> {
    let mut cards = self.cards;
    cards.sort();
    let mut ranks: Vec<u8> = Vec::new();
    for card in cards.iter() {
        ranks.append(&mut card.rank());
    }
    ranks
  }

  pub fn suits(self) -> HashMap<Suit, u8> {
      let mut suits: HashMap<Suit, u8>  = HashMap::new();
      let cards = self.cards;
      for card in cards {
          let number = suits.entry(card.suit).or_insert(0);
          *number += 1;
      }
      suits
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
  fn should_give_ranks() {
    let hand: Hand = Hand::new(vec![
      Card{suit:Suit::Hearts, rank:14}, 
      Card{suit:Suit::Diamonds, rank:2}, 
      Card{suit:Suit::Hearts, rank:3}, 
      Card{suit:Suit::Hearts, rank:7}, 
      Card{suit:Suit::Hearts, rank:10}
    ]);
    assert_eq!(hand.ranks(), vec![14, 1, 10, 7, 3, 2])
  }

  #[test]
  fn should_create_a_pair() {
    let mut hand: Hand = Hand::new(vec![
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Diamonds, rank:2}, 
      Card{suit:Suit::Hearts, rank:4}, 
      Card{suit:Suit::Hearts, rank:5}, 
      Card{suit:Suit::Hearts, rank:8}
    ]);
    hand.get_value();
    assert_eq!(hand.value.unwrap().rank, Ranks::OnePair)
  }
}
