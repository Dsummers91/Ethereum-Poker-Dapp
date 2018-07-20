#![allow(dead_code, unused_imports)]
mod dealer;
mod test;

use card::{Card, suit};
use game_types::{GameTypes};
use deck::{Deck};
use player::{Player};
use self::dealer::Deal;

#[derive(Debug)]
pub struct Table<'a, 'b, 'c : 'b, 'd : 'c> {
    pub game: GameTypes,
    deck: &'a mut Vec<Card>,
    round: u8,
    board: Vec<Card>,
    players:&'b mut [Option<&'c mut Player<'d, 'd>>]
}


impl<'a, 'b, 'c, 'd> Table<'a, 'b, 'c, 'd> {
    pub fn new(
        game: GameTypes, 
        deck: &'a mut Vec<Card>, 
        players: &'b mut[Option<&'c mut Player<'d, 'd>>], 
        _seats: usize
    ) -> Table<'a, 'b, 'c, 'd> {
        let mut board = vec![];
        Table{game, round: 0, board, deck, players}
    }
}

