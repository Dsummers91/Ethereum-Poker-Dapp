use game_types::GameTypes;
use table::{Table};

pub trait Deal {
    fn deal(&mut self);
}


impl<'a, 'b, 'c, 'd> Deal for Table<'a, 'b, 'c, 'd> {
    fn deal(&mut self) {
        println!("{:?}", self.deck);
        let card = self.deck.remove(1);
        self.board.push(card);
        let game_type = &self.game;
        self.round = self.round + 1;
        println!("{:?}", game_type);
    }
}
