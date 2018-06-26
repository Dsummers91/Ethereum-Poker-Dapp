use card::{Card};
use table::{Table};
use hand::{Hand};

#[derive(Debug)]
pub struct Player {
  pub hand: Hand,
  pub chips: u64,
}

impl Player {
    
}

#[cfg(test)]
  mod tests {
//  use super::*;
    #[test]
    fn should_be_true() {
      assert_eq!(2 + 2, 4);
    }
}

