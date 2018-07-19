#![allow(dead_code, unused_imports)]

use card::{Card};
use game_types::{GameTypes};
use deck::{Deck};
use player::{Player};
use dealer::Deal;

#[derive(Debug)]
pub struct Table<'a, 'b, 'c : 'b, 'd : 'c> {
    pub game: GameTypes,
    deck: &'a mut Vec<Card>,
    players:&'b mut [Option<&'c mut Player<'d, 'd>>]
}


impl<'a, 'b, 'c, 'd> Table<'a, 'b, 'c, 'd> {
    pub fn new(
        game: GameTypes, 
        deck: &'a mut Vec<Card>, 
        players: &'b mut[Option<&'c mut Player<'d, 'd>>], 
        _seats: usize) -> Table<'a, 'b, 'c, 'd> {
        Table{game, deck, players}
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn should_be_texas_holdem() {
        let mut deck = Deck::new();
        let player: Option<&mut Player> = None;
        let player_2: Option<&mut Player> = None;
        // TODO - Implement way to Copy (NONE) without deriving Copy
        // prolly with macros
        // this fails:
        const SEATS: usize = 2;
        //let mut players: [Option<&mut Player>; SEATS] = [Player::none(); SEATS];
        let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
        table.deal();
		assert_eq!(table.game, GameTypes::TexasHoldem);
	}
}
