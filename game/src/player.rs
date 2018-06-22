use card::{Card};
use table::{Table};
use hand::{Hand};

pub struct Player {
  pub hand: Hand,
  pub chips: u64,
}

impl Player {
  fn sit(_table: &mut Table, _seat: u8) {
    
  }
}

#[cfg(test)]
  mod tests {
//  use super::*;
    #[test]
    fn should_be_true() {
      assert_eq!(2 + 2, 4);
    }
}

