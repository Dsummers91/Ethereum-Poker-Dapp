use card::{Card, Suit};
use std::collections::HashMap;


pub enum Ranks {
  High_Card,
  One_Pair,
  Two_Pair,
  Three_of_a_Kind,
  Straight,
  Flush,
  Full_House,
  Four_of_a_Kind,
  Straight_Flush, // Royal Flush Included
}

pub fn get_rank(card: Vec<Card>) -> Ranks {
  Ranks::High_Card
}

// Should return flush
fn is_flush(cards: Vec<Card>) -> Option<Suit> {
	let mut suits: HashMap<Suit, u8>  = HashMap::new();
	for card in cards {
		let number = suits.entry(card.suit).or_insert(0);
		*number += 1;
	};
	//match {
	//	true => Some(Suit::Spades),
	//	_ => None
	//}
	// Check to see if is any suits >= 5
	None
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_be_a_flush() {
      let cards = vec![Card{suit:Suit::Spades, rank:14}, Card{suit:Suit::Spades, rank:14}];
      assert_eq!(is_flush(cards), Some(Suit::Spades));
    }
}
