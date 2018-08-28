#[cfg(test)]
mod tests {
    use super::*;
    use deck::*;
    use card;

    #[test]
    fn should_have_52_cards() {
        let deck: Vec<Card> = Deck::new();
        assert_eq!(deck.len(), 52);
    }
 
    #[test]
    fn should_be_able_to_use_global_cards() {
        let card: Vec<Card> = card::create();
        assert_eq!(true, true);
    }
}
