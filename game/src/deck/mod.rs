use card::{Card, Suit, Cards};
use rand::{thread_rng, Rng};

mod test;

pub trait Deck<'a, T> {
    fn new() -> Self;
    fn allocate(&'a [&'a Card]) -> Self;
    fn populate(&mut self);
    fn shuffle(&mut self);
}

impl<'a> Deck<'a, &'a[&'a Card]> for &'a mut [&'a Card] {
    fn new() -> Self {
        unimplemented!()
    }

    fn populate(&mut self) {
        unimplemented!()
    }

    fn allocate(cards: &'a[&'a Card]) -> Self {
        unimplemented!()
    }

    fn shuffle(mut self: &mut Self)  {
        let mut rng = thread_rng();
        rng.shuffle(&mut self);
        rng.shuffle(&mut self);
        rng.shuffle(&mut self);
    }
}

impl<'a> Deck<'a, Vec<Card>> for Vec<Card> {
    fn new() -> Self {
        let mut deck: Vec<Card> = Vec::new();
        deck.populate();
        deck
    }

    fn allocate(cards: &'a [&'a Card]) -> Self {
        unimplemented!()
    }

    fn populate(&mut self) {
      for suit in Suit::iter() {
        for rank in 2..14+1 {
          self.push(Card::new(rank, Suit::get_suit(*suit)));
        }
      }
    }

    fn shuffle(mut self: &mut Self)  {
        let mut rng = thread_rng();
        rng.shuffle(&mut self);
        rng.shuffle(&mut self);
        rng.shuffle(&mut self);
    }
}

