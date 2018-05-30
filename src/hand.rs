use card::{Card, Suit};

pub struct Hand {
  pub cards: Vec<Card>,
  pub rank: i8
}

impl Hand {
  fn new(cards: Vec<Card>) -> Hand {
    Hand{rank: 0, cards: cards}
  }

  fn get_rank(self: &mut Hand) {
    self.rank = 1;
    // Royal Flush
    // Staight Flush
    // 4 of a kind
    // Full House
    // Flush
    // Straight
    // Three of a kind
    // Two Pair
    // Pair
    // High Card
  }

}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn should_create_a_hand() {
    let hand = Hand::new(vec![Card{suit:Suit::Hearts, rank:2}]);
    assert_eq!(hand.cards[0], Card{suit:Suit::Hearts, rank:2});
  }

  #[test]
  fn should_get_value_from_hand() {
    let mut hand = Hand::new(vec![Card{suit:Suit::Hearts, rank:2}]);
    hand.get_rank();
    assert_eq!(hand.rank, 1);
  }
}
