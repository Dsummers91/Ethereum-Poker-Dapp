#![allow(dead_code, unused_imports)]
mod dealer;
mod test;

use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use card::{Card, suit};
use game_types::{GameTypes};
use deck::{Deck};
use player::{Player};
use self::dealer::Deal;

#[derive(Debug)]
pub struct Table<'a, 'b> {
    pub game: GameTypes,
    deck: &'a mut Vec<Card>,
    round: u8,
    board: Vec<Card>,
    seats: HashMap<i8, Player<'b>>,
}


impl<'a, 'b> Table<'a, 'b> {
    pub fn new(
        game: GameTypes, 
        deck: &'a mut Vec<Card>, 
        _seats: usize,
    ) -> Table<'a, 'b> {
        let mut board = vec![];
        let seats = HashMap::new();
        Table{game, seats, round: 0, board, deck}
    }

    pub fn assign_player(self,  player: &mut Player<'a>) -> Self {
        println!("{:?}", player);
        self
    }
}

