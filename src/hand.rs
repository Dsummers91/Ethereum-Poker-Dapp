pub trait Hand<T> {
    fn new(&mut self);
}


impl Hand<Vec<Card>> for Vec<Card> {
  fn new(self: &mut Self) {
    unimplemented!();
  }
}