pub mod suit;
mod test;

use std::fmt::{Display, Formatter, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
pub use self::suit::Suit;
use game_types::GameTypes;


#[derive(Debug, Eq, Clone)]
pub struct Card {
  pub rank: u8,
  pub suit: Suit,
}

pub trait Cards<'a, T> {
    fn ranks(self) -> Vec<u8>;
    fn suits(&self) -> HashMap<Suit, u8>;
    fn values(self) -> Vec<u8>;
    fn lowest_number(&self) -> u8;
}

pub fn create() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for suit in Suit::iter() {
        for rank in 2..14+1 {
            cards.push(Card::new(rank, Suit::get_suit(*suit)));
        }
    }
    cards
}

impl<'a> Cards<'a, Vec<&'a Card>> for Vec<&'a Card> {
    fn ranks(self) -> Vec<u8> {
        let mut cards: Vec<u8> = vec![];
        for card in (&self).iter() {
            cards.push(card.rank)
        }
        cards.sort();
        cards.reverse(); // highest cards first
        cards
    }
    
    fn lowest_number(&self) -> u8 {
       let cards = self.clone();
       let ranks = cards.ranks();
       // Only need this for straights, so check for a 2
       // TODO: this way won't work for Omaha Hi/Lo or Razz
       if ranks.contains(&14) && ranks.contains(&2) { 
           return 1;
       }
       self.last().unwrap().rank
    }
    
    // Values have Ace = 1 and 14
    fn values(self) -> Vec<u8> {
        let mut cards: Vec<u8> = vec![];
        for card in (&self).iter() {
            cards.append(&mut card.rank().iter().map(|c| c.0).collect())
        }
        cards
    }

    fn suits(&self) -> HashMap<Suit, u8> {
        let mut suits: HashMap<Suit, u8>  = HashMap::new();
        for card in (&self).iter() {
            let number = suits.entry(card.suit).or_insert(0);
            *number += 1;
        }
        suits
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
        if self.rank == 14 {
            return 14 == other.rank || 1 == other.rank
        } 
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

  pub fn rank(self: &Card) -> Vec<(u8, &Card)> {
    if self.rank != 14 {
      vec![(self.rank, self)]
    } else {
      vec![(14, self), (1, self)]
    }
  }
}


