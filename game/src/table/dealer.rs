use game_types::GameTypes;
use table::{Table};

pub trait Deal {
    fn deal(&mut self);
    fn show_flop(&mut self);
}


impl<'a, 'b> Deal for Table<'a, 'b> {
    fn deal(&mut self) {
        unimplemented!()
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
