use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;


#[derive(Debug)]
pub struct Card {
  pub suit: Suit,
  pub rank: u8
}

#[derive(Debug)]
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
        &Suit::Spades => Suit::Spades,
        &Suit::Diamonds => Suit::Diamonds,
        &Suit::Hearts => Suit::Hearts,
        &Suit::Clubs => Suit::Clubs,
      }
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
    Card{rank: rank, suit: suit}
  }

  pub fn value(self: &Card) -> Vec<u8> {
    if self.rank != 14 {
      vec![self.rank]
    } else {
      vec![1, 14]
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;
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
    fn deuce_should_have_value_of_2() {
      let card = Card{suit:Suit::Hearts, rank:2};
      assert_eq!(card.value(), vec![2]);
    }

    #[test]
    fn ace_should_have_value_if_1_or_14() {
      let card = Card{suit:Suit::Hearts, rank:14};
      assert_eq!(card.value(), vec![1, 14]);
    }

    #[test]
    fn king_should_have_value_if_13() {
      let card = Card{suit:Suit::Hearts, rank:13};
      assert_eq!(card.value(), vec![13]);
    }
}
