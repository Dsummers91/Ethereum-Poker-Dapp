use card::{Card};
use table::{Table};
use hand::{Hand};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Player<'a, T: 'a> {
    pub hand: Hand<'a>,
    pub chips: u64,
    phantom: PhantomData<&'a T>,
}

#[cfg(test)]
mod tests {
    //  use super::*;
    #[test]
    fn should_be_true() {
        assert_eq!(2 + 2, 4);
    }
}

