use card::{Card};
use deck::{Deck};
use table::{Table};

pub struct Player {
  pub hand: Vec<Card>,
  pub chips: u64,
}

impl Player {
  fn sit(table: &mut Table, seat: u8) {

  }
}

#[cfg(test)]
  mod tests {
  use super::*;
    #[test]
    fn should_be_true() {
      assert_eq!(2 + 2, 4);
    }
}

