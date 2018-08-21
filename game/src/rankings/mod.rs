#![allow(dead_code)]

use card::{Card, Suit, Cards};
use hand::{Hand};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rank<'a> {
    pub rank: Ranks,
    pub cards: Vec<&'a Card>
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Ranks {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush, // Royal Flush Included
}

pub fn get_rank(_card: &Hand) -> Option<Ranks> {
    if let Some(_x) = is_pair(_card.used_cards.to_vec()) {
        return Some(Ranks::OnePair)
    }
    None
}

// Should return flush
fn is_flush<'a>(cards: Vec<&'a Card>) -> Option<Rank<'a>> {
    let c = cards.clone();
    let suits = Cards(c).suits();
    if let Some(count_hearts) = suits.get(&Suit::Hearts) {
        if *count_hearts >= 5 {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(cards, Suit::Hearts)})
        }
    }
    if let Some(count_diamonds) = suits.get(&Suit::Diamonds) {
        if *count_diamonds >= 5 {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(cards, Suit::Diamonds)})
        }
    }
    if let Some(count_spades) = suits.get(&Suit::Spades) {
        if *count_spades >= 5 {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(cards, Suit::Spades)})
        }
    }
    if let Some(count_clubs) = suits.get(&Suit::Clubs) {
        if *count_clubs >= 5 {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(cards, Suit::Clubs)})
        }
    }
    None
}


#[allow(unknown_lints, needless_pass_by_value)]
fn get_flush_cards(cards: Vec<&Card>, suit: Suit) -> Vec<&Card> {
    cards.iter().filter(|card| card.suit == suit).map(|card| *card).collect()
}

fn get_flush_hand(mut cards: Vec<&Card>, suit: Suit) -> Vec<&Card> {
    cards.sort();
    cards.iter().filter(|card| card.suit == suit).take(5).map(|card| *card).collect()
}


fn is_straight_flush(c: Vec<&Card>) -> Option<u8> {
    let cards = Cards(c.clone());
    let suits = cards.suits();
    if let Some(count_hearts) = suits.get(&Suit::Hearts) {
        if *count_hearts >= 5 {
            let result = get_flush_cards(c, Suit::Hearts);
            return is_straight(result)
        }
    }
    if let Some(count_diamonds) = suits.get(&Suit::Diamonds) {
        if *count_diamonds >= 5 {
            return is_straight(get_flush_cards(c, Suit::Diamonds))
        }
    }
    if let Some(count_spades) = suits.get(&Suit::Spades) {
        if *count_spades >= 5 {
            return is_straight(get_flush_cards(c, Suit::Spades))
        }
    }
    if let Some(count_clubs) = suits.get(&Suit::Clubs) {
        if *count_clubs >= 5 {
            return is_straight(get_flush_cards(c, Suit::Clubs))
        }
    }
    None
}

fn is_straight(mut c: Vec<&Card>) -> Option<u8> {
    c.sort();
    for window in c.windows(5) {
        let mut cards = Cards(window.to_vec());
        let low_card = cards.lowest_number();
        let ranks = cards.values();
        if ranks.contains(&(low_card as u8)) 
            && ranks.contains(&(low_card + 1 as u8)) 
            && ranks.contains(&(low_card + 2 as u8)) 
            && ranks.contains(&(low_card + 3 as u8)) 
            && ranks.contains(&(low_card + 4 as u8)) {
                return Some(low_card + 4)
            }
    }
    None
}

fn is_pair(mut cards: Vec<&Card>) -> Option<u8> {
    cards.sort();
    let ranks = Cards(cards).ranks();
    for rank in ranks.windows(2) {
        println!("{:?}", rank);
        if rank[0] == rank[1] {
            return Some(rank[0]);
        }
    }
    None
}

fn is_two_pair(mut cards: Vec<&Card>) -> Option<[u8; 2]> {
    cards.sort();
    let mut leftover_cards = cards.clone();
    if let Some(x) = is_pair(cards) {
        leftover_cards = leftover_cards.iter().filter(|r| r.rank != x).cloned().collect();
        if let Some(y) = is_pair(leftover_cards) {
            return Some([x, y])
        }
    } 
    None
}

fn is_full_house(cards: Vec<&Card>) -> Option<[u8; 2]> {
    let mut leftover_cards = cards.clone();
    if let Some(x) = is_three_of_a_kind(cards) {
        leftover_cards = leftover_cards.iter().filter(|r| r.rank != x).cloned().collect();
        if let Some(y) = is_pair(leftover_cards.to_vec()) {
            return Some([x, y])
        }
    } 
    None
}

fn is_three_of_a_kind(cards: Vec<&Card>) -> Option<u8> {
    let ranks = Cards(cards).ranks();
    for rank in ranks.windows(3) {
        println!("{:?}", rank);
        if rank[0] == rank[1] && rank[0] == rank[2] {
            return Some(rank[0]);
        }
    }
    None
}

fn is_four_of_a_kind(cards: Vec<&Card>) -> Option<u8> {
    let ranks = Cards(cards).ranks();
    for rank in ranks.windows(4) {;
        if rank[0] == rank[1] && rank[0] == rank[2] && rank[0] == rank[3] {
            return Some(rank[0]);
        }
    }
    None
}

fn high_card(cards: &[u8]) -> Option<u8> {
    Some(*cards.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_pair_of_deuces() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        assert_eq!(is_pair(cards), Some(2));
    }

    #[test]
    fn should_not_be_a_fullhouse() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:3} 
        ];
        assert_eq!(is_full_house(cards), None);
    }

    #[test]
    fn should_be_two_pair_5_and_10() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:10} 
        ];
        assert_eq!(is_two_pair(cards), Some([10, 5]));
    }

    #[test]
    fn should_be_a_fullhouse_3_over_2() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3} 
        ];
        assert_eq!(is_full_house(cards), Some([3, 2]));
    }

    #[test]
    fn should_be_a_three_of_a_kind_queens() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:6}, 
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:12}
        ];
        assert_eq!(is_three_of_a_kind(cards), Some(12));
    }

    #[test]
    fn should_be_a_four_of_a_kind_9() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:2} 
        ];
        assert_eq!(is_four_of_a_kind(cards), Some(9));
    }

    #[test]
    fn should_be_a_straight_7_high() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards.to_vec());
        assert_eq!(is_straight(hand.used_cards.to_vec()), Some(7));
    }

    #[test]
    fn should_be_a_straight_ace_high() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:13}, 
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:11}, 
            &Card{suit:Suit::Hearts, rank:10}
        ];
        let hand = Hand::new(cards.to_vec());
        assert_eq!(is_straight(hand.used_cards.to_vec()), Some(14));
    }

    #[test]
    fn should_be_a_straight_five_high() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:5}
        ];
        let hand = Hand::new(cards.to_vec());
        assert_eq!(is_straight(hand.used_cards.to_vec()), Some(5));
    }

    #[test]
    fn should_be_a_hearts_flush() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let c = cards.clone();
        let flush_result = is_flush(c).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
        let mut iter = flush_result.cards.iter();
        assert_eq!(&cards[0], iter.next().unwrap());
        assert_eq!(&cards[1], iter.next().unwrap());
        assert_eq!(&cards[4], iter.next().unwrap());
        assert_eq!(&cards[3], iter.next().unwrap());
        assert_eq!(&cards[2], iter.next().unwrap());
    }

    #[test]
    fn should_return_highest_five_flush() {
        let cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:8}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let c = cards.clone();
        let mut flush = get_flush_cards(c.to_vec(), Suit::Hearts);
        let high_flush_cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:8}, 
            &Card{suit:Suit::Hearts, rank:6},
            &Card{suit:Suit::Hearts, rank:4}, 
        ];
        flush.sort();
        assert_eq!(flush[0..5], high_flush_cards);
    }

    #[test]
    fn should_be_a_straight_flush_five_high() {
        let cards = [
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:4}
        ];
        let c = cards.clone();
        let flush_result = is_straight_flush(c.to_vec());
        assert_eq!(flush_result, Some(5));
    }

    #[test]
    fn should_be_a_straight_flush_ace_high() {
        let cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:13}, 
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:11}
        ];
        assert_eq!(is_straight_flush(cards.to_vec()), Some(14));
    }

    #[test]
    fn should_be_a_straight_flush_jack_high() {
        let cards = [
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:8}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:11}
        ];
        assert_eq!(is_straight_flush(cards.to_vec()), Some(11));
    }

    #[test]
    fn should_not_be_a_straight_flush() {
        let cards = [
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Diamonds, rank:6}, 
            &Card{suit:Suit::Hearts, rank:7}
        ];
        assert_eq!(is_straight_flush(cards.to_vec()), None);
    }

    #[test]
    fn should_be_a_diamonds_flush() {
        let cards = vec![
            &Card{suit:Suit::Diamonds, rank:14}, 
            &Card{suit:Suit::Diamonds, rank:10}, 
            &Card{suit:Suit::Diamonds, rank:2}, 
            &Card{suit:Suit::Diamonds, rank:4}, 
            &Card{suit:Suit::Diamonds, rank:6}
        ];
        let flush_result = is_flush(cards).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_be_a_clubs_flush() {
        let cards = vec![
            &Card{suit:Suit::Clubs, rank:14}, 
            &Card{suit:Suit::Clubs, rank:10}, 
            &Card{suit:Suit::Clubs, rank:2}, 
            &Card{suit:Suit::Clubs, rank:4}, 
            &Card{suit:Suit::Clubs, rank:6}
        ];
        let flush_result = is_flush(cards).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_be_a_spades_flush() {
        let cards = vec![
            &Card{suit:Suit::Spades, rank:14}, 
            &Card{suit:Suit::Spades, rank:10}, 
            &Card{suit:Suit::Spades, rank:2}, 
            &Card{suit:Suit::Spades, rank:4}, 
            &Card{suit:Suit::Spades, rank:6}
        ];
        let flush_result = is_flush(cards).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_not_be_a_flush_1() {
        let cards = vec![
            &Card{suit:Suit::Spades, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Spades, rank:2}, 
            &Card{suit:Suit::Spades, rank:4}, 
            &Card{suit:Suit::Spades, rank:6}
        ];
        let flush_result = is_flush(cards);
        assert_eq!(flush_result, None);
    }

    #[test]
    fn should_not_be_a_flush_2() {
        let cards = vec![
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Clubs, rank:5}, 
            &Card{suit:Suit::Clubs, rank:2}, 
            &Card{suit:Suit::Diamonds, rank:5}
        ];
        let flush_result = is_flush(cards);
        assert_eq!(flush_result, None);
    }


}
