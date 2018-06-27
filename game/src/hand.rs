use card::{Card, Suit};
use rankings::{get_rank, Ranks, Rank};
use std::collections::HashMap;
use std;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand<'a> {
   pub cards: &'a mut[&'a Card],
   value: Option<Rank>
}

impl<'a> Hand<'a> {
  pub fn new(cards: &'a mut [&'a Card]) -> Hand<'a> {
    Hand{cards, value: None}
  }

  pub fn get_value(&mut self) {
    if let Some(x) = get_rank(&self) {
        self.value = Some(Rank{active_cards: vec![], rank: Ranks::HighCard}); 
    }
  }

  pub fn ranks(&mut self) -> Vec<u8> {
    self.cards.sort();
    let mut ranks: Vec<u8> = Vec::new();
    for card in self.cards.iter() {
        ranks.append(&mut card.rank());
    }
    ranks
  }

  pub fn suits(&self) -> HashMap<Suit, u8> {
      let mut suits: HashMap<Suit, u8>  = HashMap::new();
      for card in self.cards.iter() {
          let number = suits.entry(card.suit.clone()).or_insert(0);
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
    let mut cards = vec![&Card{suit:Suit::Hearts, rank:2}];
    let mut hand: Hand = Hand::new(&mut cards);
    assert_eq!(hand, hand)
  }

  #[test]
  fn should_give_ranks() {
	let mut cards = vec![
      &Card{suit:Suit::Hearts, rank:14},
      &Card{suit:Suit::Diamonds, rank:2},
      &Card{suit:Suit::Hearts, rank:3},
      &Card{suit:Suit::Hearts, rank:7},
      &Card{suit:Suit::Hearts, rank:10}
    ];
    let mut hand: Hand = Hand::new(&mut cards);
    assert_eq!(hand.ranks(), vec![14, 1, 10, 7, 3, 2])
  }

  #[test]
  fn should_create_a_pair() {
	let mut cards = vec![
      &Card{suit:Suit::Hearts, rank:2},
      &Card{suit:Suit::Diamonds, rank:2},
      &Card{suit:Suit::Hearts, rank:4},
      &Card{suit:Suit::Hearts, rank:5},
      &Card{suit:Suit::Hearts, rank:8}
    ];
    let mut hand: Hand = Hand::new(&mut cards);
    hand.get_value();
    assert_eq!(hand.value.unwrap().rank, Ranks::OnePair)
  }
}
