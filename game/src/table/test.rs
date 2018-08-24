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

	#[test]
    fn player_should_be_able_to_sit() {
        let mut deck: Vec<Card> = Deck::new();
        let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, 6);
        let cards = vec![];
        let hand = Hand::new(cards);
        let mut player = Player{hand: hand, chips: 1000, active: false, sitting_out: false};
        table.assign_player(player, 1);
        assert_eq!(table.seats.lock().unwrap().get(&1).unwrap().chips, 1000);
    }

	#[test]
	fn should_be_able_to_deal_to_players() {
        let mut deck: Vec<Card> = Deck::new();
        let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, 6);
        let cards = vec![];
        let hand = Hand::new(cards);
        let mut player = Player::new(hand);
        table.assign_player(player, 1);
        player.add_chips(1000);
        table.start_hand();
        println!("{:?}", table);
    }
}
