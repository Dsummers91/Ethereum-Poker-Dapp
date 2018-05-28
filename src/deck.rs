use card::{Card, Suit};
use rand::{thread_rng, Rng};

pub trait Deck<T> {
    fn new() -> Self;
    fn populate(&mut self);
    fn shuffle(&mut self);
}

impl Deck<Vec<Card>> for Vec<Card> {
    fn new() -> Vec<Card> {
        let mut deck: Vec<Card> = Vec::new();
        deck.populate();
        deck.shuffle(); // How random is this anyway?
        deck
    }

    fn populate(self: &mut Self) {
      for suit in Suit::iter() {
        for rank in 2..14+1 {
          self.push(Card::new(rank, Suit::get_suit(suit)));
        }
      }
    }

    fn shuffle(mut self: &mut Self)  {
        let mut rng = thread_rng();
        rng.shuffle(&mut self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_be_true() {
        let deck: Vec<Card> = Deck::new();
        assert_eq!(deck.len(), 52);
    }
}
