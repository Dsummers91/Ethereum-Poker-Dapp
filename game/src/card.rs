use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;
use std::hash::{Hash};
use std::cmp::Ordering;


#[derive(Debug, Eq, Clone)]
pub struct Card {
  pub rank: u8,
  pub suit: Suit
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Suit {
  Spades,
  Diamonds,
  Clubs,
  Hearts,
}

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit;  4] = [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts];
        SUITS.into_iter()
    }

    pub fn get_suit(item: &Suit) -> Suit {
      match item {
        Suit::Spades => Suit::Spades,
        Suit::Diamonds => Suit::Diamonds,
        Suit::Hearts => Suit::Hearts,
        Suit::Clubs => Suit::Clubs,
      }
    }
}
impl Ord for Card {
    fn cmp(&self,  other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self,  other: &Card) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

impl Display for Card {
  fn fmt(self: &Card, f: &mut Formatter) -> Result {
    let rank = match self.rank {
      2   => "Deuce",
      3   => "3",
      4   => "4",
      5   => "5",
      6   => "6",
      7   => "7",
      8   => "8",
      9   => "9",
      10  => "10",
      11  => "Jack",
      12  => "Queen",
      13  => "King",
      14  => "Ace",
      _ => panic!("Rank does not exist")
    };
    write!(f, "{} of {:?}", rank, self.suit)
  }
}

impl Card {
  pub fn new(rank: u8, suit: Suit) -> Card {
    Card{rank, suit}
  }

  pub fn rank(self: &Card) -> Vec<u8> {
    if self.rank != 14 {
      vec![self.rank]
    } else {
      vec![14, 1]
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Suit;

    #[test]
    fn should_sort_correctly() {
        let mut cards = vec![
            &Card{suit:Suit::Spades, rank: 13},
			&Card{suit:Suit::Diamonds, rank: 12},
			&Card{suit:Suit::Spades, rank: 14},
		];
		let correct_order = vec![
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 13},
			&Card{suit:Suit::Spades, rank: 12},
		];
		cards.sort();
		assert_eq!(cards, correct_order); 
	}

    #[test]
    fn should_format_correctly() {
      let card = Card{suit:Suit::Spades, rank:14};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Ace of Spades");
    }


    #[test]
    fn should_format_correctly2() {
      let card = Card{suit:Suit::Clubs, rank:2};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Deuce of Clubs");
    }

    #[test]
    fn should_format_correctly3() {
      let card = Card{suit:Suit::Diamonds, rank:5};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "5 of Diamonds");
    }


    #[test]
    fn should_format_correctly4() {
      let card = Card{suit:Suit::Hearts, rank:13};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "King of Hearts");
    }

    #[test]
    fn deuce_should_have_rank_of_2() {
      let card = Card{suit:Suit::Hearts, rank:2};
      assert_eq!(card.rank(), vec![2]);
    }

    #[test]
    fn ace_should_have_rank_if_1_or_14() {
      let card = Card{suit:Suit::Hearts, rank:14};
      assert_eq!(card.rank(), vec![14, 1]);
    }

    #[test]
    fn king_should_have_rank_if_13() {
      let card = Card{suit:Suit::Hearts, rank:13};
      assert_eq!(card.rank(), vec![13]);
    }
}
