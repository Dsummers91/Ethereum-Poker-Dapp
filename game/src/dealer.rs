use game_types::GameTypes;
use table::{Table};

pub trait Deal {
    fn deal(&mut self);
}


impl<'a, 'b, 'c, 'd> Deal for Table<'a, 'b, 'c, 'd> {
    fn deal(&mut self) {
        let game_type = &self.game;
        println!("{:?}", game_type);
    }
}
