use game_types::GameTypes;
use table::{Table};

pub trait Deal {
    fn deal(&mut self);
    fn show_flop(&mut self);
}


impl<'a, 'b, 'c, 'd> Deal for Table<'a, 'b, 'c, 'd> {
    fn deal(&mut self) {
        for player in self.players.iter() {
            let card = self.deck.remove(1);
            self.board.push(card);
            println!("{:?}",  player);
        }
        let game_type = &self.game;
        self.round = self.round + 1;
        println!("{:?}", game_type);
    }

    fn show_flop(&mut self) {
        println!("{:?}", self.deck);
        let card = self.deck.remove(1);
        self.board.push(card);
        let game_type = &self.game;
        self.round = self.round + 1;
        println!("{:?}", game_type);
    }
}
