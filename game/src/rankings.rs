use card::{Card, Suit};
use std::collections::HashMap;


pub enum Ranks {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  Straight,
  Flush,
  FullHouse,
  FourOfAKind,
  StraightFlush, // Royal Flush Included
}

pub fn get_rank(_card: Vec<Card>) -> Ranks {
    Ranks::HighCard
}

// Should return flush
fn is_flush(cards: Vec<Card>) -> Option<Suit> {
    let mut suits: HashMap<Suit, u8>  = HashMap::new();
    for card in cards {
        let number = suits.entry(card.suit).or_insert(0);
        *number += 1;
    }
    match suits {
        _ if *suits.get(&Suit::Spades).unwrap_or(&0) >= 5 => Some(Suit::Spades),
        _ if *suits.get(&Suit::Clubs).unwrap_or(&0) >= 5 => Some(Suit::Clubs),
        _ if *suits.get(&Suit::Hearts).unwrap_or(&0) >= 5 => Some(Suit::Hearts),
        _ if *suits.get(&Suit::Diamonds).unwrap_or(&0) >= 5 => Some(Suit::Diamonds),
        _ => None
    }
}

fn is_straight(_cards: Vec<Card>) -> Option<Suit> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_be_a_hearts_flush() {
      let cards = vec![
          Card{suit:Suit::Hearts, rank:14}, 
          Card{suit:Suit::Hearts, rank:10}, 
          Card{suit:Suit::Hearts, rank:2}, 
          Card{suit:Suit::Hearts, rank:4}, 
          Card{suit:Suit::Hearts, rank:6}
      ];
      assert_eq!(is_flush(cards), Some(Suit::Hearts));
    }

    #[test]
    fn should_be_a_diamonds_flush() {
      let cards = vec![
          Card{suit:Suit::Diamonds, rank:14}, 
          Card{suit:Suit::Diamonds, rank:10}, 
          Card{suit:Suit::Diamonds, rank:2}, 
          Card{suit:Suit::Diamonds, rank:4}, 
          Card{suit:Suit::Diamonds, rank:6}
      ];
      assert_eq!(is_flush(cards), Some(Suit::Diamonds));
    }

    #[test]
    fn should_be_a_clubs_flush() {
      let cards = vec![
          Card{suit:Suit::Clubs, rank:14}, 
          Card{suit:Suit::Clubs, rank:10}, 
          Card{suit:Suit::Clubs, rank:2}, 
          Card{suit:Suit::Clubs, rank:4}, 
          Card{suit:Suit::Clubs, rank:6}
      ];
      assert_eq!(is_flush(cards), Some(Suit::Clubs));
    }

    #[test]
    fn should_be_a_spades_flush() {
      let cards = vec![
          Card{suit:Suit::Spades, rank:14}, 
          Card{suit:Suit::Spades, rank:10}, 
          Card{suit:Suit::Spades, rank:2}, 
          Card{suit:Suit::Spades, rank:4}, 
          Card{suit:Suit::Spades, rank:6}
      ];
      assert_eq!(is_flush(cards), Some(Suit::Spades));
    }

    #[test]
    fn should_not_be_a_flush_1() {
      let cards = vec![
          Card{suit:Suit::Spades, rank:14}, 
          Card{suit:Suit::Hearts, rank:10}, 
          Card{suit:Suit::Spades, rank:2}, 
          Card{suit:Suit::Spades, rank:4}, 
          Card{suit:Suit::Spades, rank:6}
      ];
      assert_eq!(is_flush(cards), None);
    }

    #[test]
    fn should_not_be_a_flush_2() {
      let cards = vec![
          Card{suit:Suit::Hearts, rank:5}, 
          Card{suit:Suit::Hearts, rank:2}, 
          Card{suit:Suit::Clubs, rank:5}, 
          Card{suit:Suit::Clubs, rank:2}, 
          Card{suit:Suit::Diamonds, rank:5}
      ];
      assert_eq!(is_flush(cards), None);
    }
}
