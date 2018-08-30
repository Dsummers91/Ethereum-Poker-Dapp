#[cfg(test)]
mod tests {
    use super::*;
    use deck::*;
    use card;

    #[test]
    fn should_have_52_cards() {
    }
 
    #[test]
    fn should_be_able_to_use_global_cards() {
        let cards: Vec<Card> = card::create();
        let card_ref = card::allocate(&cards);
        let deck: &[&Card] = &card_ref[..];
    }
}
