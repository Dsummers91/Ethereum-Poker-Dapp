
#[cfg(test)]
mod tests {
  use hand::Hand;
  use card::{Card, suit:: Suit};
  use rankings::Ranks;

  #[test]
  fn should_create_a_hand() {
    let mut cards = vec![&Card{suit:Suit::Hearts, rank:2}];
    let hand: Hand = Hand::new(&mut cards);
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
    assert_eq!(hand.ranks(), vec![14, 10, 7, 3, 2])
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
