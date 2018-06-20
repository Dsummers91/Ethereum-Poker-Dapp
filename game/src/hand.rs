use card::{Card, Suit};

#[derive(Debug, Eq, PartialEq)]

pub struct Hand<Card> (Vec<Card>);

impl Hand<Card> {
  fn new(cards: Vec<Card>) -> Hand<Card> {
    Hand(cards)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::Suit;
  #[test]
  fn should_create_a_hand() {
    let hand: Hand<Card> = Hand::new(vec![Card{suit:Suit::Hearts, rank:2}]);
    assert_eq!(hand, hand)
  }

  fn should_create_a_pair() {
    let hand: Hand<Card> = Hand::new(vec![
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Diamonds, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}, 
      Card{suit:Suit::Hearts, rank:2}
    ]);
    assert_eq!(hand, hand)
  }
}
