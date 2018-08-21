use card::{Card, suit};
use game_types::{GameTypes};
use deck::{Deck};
use hand::Hand;
use player::{Player};
use table::{Table, dealer::Deal};

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn should_be_texas_holdem() {
        let mut deck = Deck::new();
        let player: Option<&mut Player> = None;
        let player_2: Option<&mut Player> = None;
        const SEATS: usize = 2;
        // TODO - Implement way to Copy (NONE) without deriving Copy
        // prolly with macros
        // this fails:
        //let mut players: [Option<&mut Player>; SEATS] = [Player::none(); SEATS];
        let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
        table.deal();
		assert_eq!(table.game, GameTypes::TexasHoldem);
	}

    fn player_should_be_able_to_sit() {
        let table = Table::new();
    }

	fn should_be_able_to_deal_to_players() {
        let mut player_hand = Hand::new(&mut []);
        let mut player_2_hand = Hand::new(&mut []);
        let mut deck: Vec<Card> = Deck::new();
        let mut player_struct =  Player::new(&mut player_hand);
        let mut player_struct_2 =  Player::new(&mut player_2_hand);
        let player: Option<&mut Player> = Some(&mut player_struct);
        let player_2: Option<&mut Player> = Some(&mut player_struct_2); 
        const SEATS: usize = 2;
        //let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		//let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
		//assert_eq!(table.game, GameTypes::TexasHoldem);
	}
}
