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
        const SEATS: usize = 2;
		let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, SEATS);
		assert_eq!(table.game, GameTypes::TexasHoldem);
	}

    fn player_should_be_able_to_sit() {
        let mut deck: Vec<Card> = Deck::new();
        let table = Table::new(GameTypes::TexasHoldem, &mut deck, 6);
        let cards = vec![];
        let hand = Hand::new(cards);
        let mut player = Player{hand: hand, chips: 1000};
        table.assign_player(&mut player);
    }

	fn should_be_able_to_deal_to_players() {
        let mut player_hand = Hand::new(vec![]);
        let mut player_2_hand = Hand::new(vec![]);
        let mut deck: Vec<Card> = Deck::new();
        let mut player_struct =  Player::new(player_hand);
        let mut player_struct_2 =  Player::new(player_2_hand);
        let player: Option<&mut Player> = Some(&mut player_struct);
        let player_2: Option<&mut Player> = Some(&mut player_struct_2); 
        const SEATS: usize = 2;
        //let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		//let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
		//assert_eq!(table.game, GameTypes::TexasHoldem);
	}
}
