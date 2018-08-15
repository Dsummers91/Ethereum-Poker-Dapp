use card::{Card, suit};
use game_types::{GameTypes};
use deck::{Deck};
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
	fn should_be_able_to_deal_to_players() {
        let mut deck = Deck::new();
        let player: Option<&mut Player> = None;
        let player_2: Option<&mut Player> = None;
        const SEATS: usize = 2;
        let mut players: [Option<&mut Player>; SEATS] = [player, player_2];
		let mut table = Table::new(GameTypes::TexasHoldem, &mut deck, &mut players, SEATS);
        table.deal();
		assert_eq!(table.game, GameTypes::TexasHoldem);
	}
}
