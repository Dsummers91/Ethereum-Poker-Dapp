use std::slice::Iter;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Suit {
  Spades,
  Diamonds,
  Clubs,
  Hearts,
}

pub static SUITS: [Suit;  4] = [Suit::Spades, Suit::Diamonds, Suit::Clubs, Suit::Hearts];

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        SUITS.into_iter()
    }

    pub fn get_suit(item: Suit) -> Suit {
      match item {
        Suit::Spades => Suit::Spades,
        Suit::Diamonds => Suit::Diamonds,
        Suit::Hearts => Suit::Hearts,
        Suit::Clubs => Suit::Clubs,
      }
    }
}
