use card::{Card, Suit};
use hand::{Hand};
use std::collections::HashMap;


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

pub fn get_rank(_card: Hand) -> Ranks {
    Ranks::HighCard
}

// Should return flush
fn is_flush(suits: HashMap<Suit, u8>) -> Option<Suit> {
    match suits {
        _ if *suits.get(&Suit::Spades).unwrap_or(&0) >= 5 => Some(Suit::Spades),
        _ if *suits.get(&Suit::Clubs).unwrap_or(&0) >= 5 => Some(Suit::Clubs),
        _ if *suits.get(&Suit::Hearts).unwrap_or(&0) >= 5 => Some(Suit::Hearts),
        _ if *suits.get(&Suit::Diamonds).unwrap_or(&0) >= 5 => Some(Suit::Diamonds),
        _ => None
    }
}

fn is_straight_flush(mut ranks: Vec<u8>, mut suits: HashMap<Suit, u8>) -> Option<u8> {
    if let Some(x) = is_flush(suits) {
        if let Some(y) = is_straight(ranks) {
            return Some(y)
        }
    }
    None
}

fn is_straight(mut cards: Vec<u8>) -> Option<u8> {
    cards.sort();
    let rank: u8 = *cards.last().unwrap();
    for i in 0..5 {
        if cards.contains(&(rank-i as u8)) {
            continue;
        } else if cards.len() > 5 {
            cards.pop();
            return is_straight(cards)
        } else {
            return None
        }
    }
    return Some(rank)
}

fn is_pair(mut ranks: Vec<u8>) -> Option<u8> {
    for rank in ranks.windows(2) {
        if rank[0] == rank[1] {
            return Some(rank[0]);
        }
    }
    None
}

fn is_two_pair(mut ranks: Vec<u8>) -> Option<[u8; 2]> {
    let mut leftover_cards = ranks.clone();
    if let Some(x) = is_pair(ranks) {
        leftover_cards = leftover_cards.iter().filter(|r| **r != x && **r > 1).cloned().collect();
        if let Some(y) = is_pair(leftover_cards.to_vec()) {
            return Some([x, y])
        }
    } 
    None
}

fn is_full_house(mut ranks: Vec<u8>) -> Option<[u8; 2]> {
    let mut leftover_cards = ranks.clone();
    if let Some(x) = is_three_of_a_kind(ranks) {
        leftover_cards = leftover_cards.iter().filter(|r| **r != x && **r > 1).cloned().collect();
        if let Some(y) = is_pair(leftover_cards.to_vec()) {
            return Some([x, y])
        }
    } 
    None
}

fn is_three_of_a_kind(mut ranks: Vec<u8>) -> Option<u8> {
    for rank in ranks.windows(3) {
        if rank[0] == rank[1] && rank[0] == rank[2] {
            return Some(rank[0]);
        }
    }
    None
}

fn is_four_of_a_kind(mut ranks: Vec<u8>) -> Option<u8> {
    for rank in ranks.windows(4) {;
        if rank[0] == rank[1] && rank[0] == rank[2] && rank[0] == rank[3] {
            return Some(rank[0]);
        }
    }
    None
}

fn high_card(ranks: Vec<u8>) -> Option<u8> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_a_pair_of_deuces() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:7}, 
            Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_pair(hand.ranks()), Some(2));
    }

    #[test]
    fn should_not_be_a_fullhouse() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:3} 
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_full_house(hand.ranks()), None);
    }

    #[test]
    fn should_be_two_pair_5_and_10() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:10} 
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_two_pair(hand.ranks()), Some([10, 5]));
    }

    #[test]
    fn should_be_a_fullhouse_3_over_2() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3} 
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_full_house(hand.ranks()), Some([3, 2]));
    }

    #[test]
    fn should_be_a_three_of_a_kind_queens() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:12}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:6}, 
            Card{suit:Suit::Hearts, rank:12}, 
            Card{suit:Suit::Hearts, rank:7}, 
            Card{suit:Suit::Hearts, rank:12}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_three_of_a_kind(hand.ranks()), Some(12));
    }

    #[test]
    fn should_be_a_four_of_a_kind_9() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:2} 
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_four_of_a_kind(hand.ranks()), Some(9));
    }

    #[test]
    fn should_be_a_straight_7_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:7}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(7));
    }

    #[test]
    fn should_be_a_straight_ace_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:13}, 
            Card{suit:Suit::Hearts, rank:12}, 
            Card{suit:Suit::Hearts, rank:11}, 
            Card{suit:Suit::Hearts, rank:10}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(14));
    }

    #[test]
    fn should_be_a_straight_five_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:5}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_straight(hand.ranks()), Some(5));
    }

    #[test]
    fn should_be_a_hearts_flush() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Hearts));
    }

    #[test]
    fn should_be_a_straight_flush_five_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:4}
        ];
        let hand = Hand::new(cards);
        let clone = hand.clone();
        assert_eq!(is_straight_flush(hand.ranks(), clone.suits()), Some(5));
    }

    #[test]
    fn should_be_a_straight_flush_ace_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:14}, 
            Card{suit:Suit::Hearts, rank:13}, 
            Card{suit:Suit::Hearts, rank:12}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Hearts, rank:11}
        ];
        let hand = Hand::new(cards);
        let clone = hand.clone();
        assert_eq!(is_straight_flush(hand.ranks(), clone.suits()), Some(14));
    }

    #[test]
    fn should_be_a_straight_flush_jack_high() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:7}, 
            Card{suit:Suit::Hearts, rank:8}, 
            Card{suit:Suit::Hearts, rank:9}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Hearts, rank:11}
        ];
        let hand = Hand::new(cards);
        let clone = hand.clone();
        assert_eq!(is_straight_flush(hand.ranks(), clone.suits()), Some(11));
    }

    #[test]
    fn should_not_be_a_straight_flush() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Hearts, rank:3}, 
            Card{suit:Suit::Hearts, rank:4}, 
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Diamonds, rank:6}, 
            Card{suit:Suit::Hearts, rank:7}
        ];
        let hand = Hand::new(cards);
        let clone = hand.clone();
        assert_eq!(is_straight_flush(hand.ranks(), clone.suits()), None);
    }

    #[test]
    fn should_be_a_diamonds_flush() {
        let cards = vec![
            Card{suit:Suit::Diamonds, rank:14}, 
            Card{suit:Suit::Diamonds, rank:10}, 
            Card{suit:Suit::Diamonds, rank:2}, 
            Card{suit:Suit::Diamonds, rank:4}, 
            Card{suit:Suit::Diamonds, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Diamonds));
    }

    #[test]
    fn should_be_a_clubs_flush() {
        let cards = vec![
            Card{suit:Suit::Clubs, rank:14}, 
            Card{suit:Suit::Clubs, rank:10}, 
            Card{suit:Suit::Clubs, rank:2}, 
            Card{suit:Suit::Clubs, rank:4}, 
            Card{suit:Suit::Clubs, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Clubs));
    }

    #[test]
    fn should_be_a_spades_flush() {
        let cards = vec![
            Card{suit:Suit::Spades, rank:14}, 
            Card{suit:Suit::Spades, rank:10}, 
            Card{suit:Suit::Spades, rank:2}, 
            Card{suit:Suit::Spades, rank:4}, 
            Card{suit:Suit::Spades, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), Some(Suit::Spades));
    }

    #[test]
    fn should_not_be_a_flush_1() {
        let cards = vec![
            Card{suit:Suit::Spades, rank:14}, 
            Card{suit:Suit::Hearts, rank:10}, 
            Card{suit:Suit::Spades, rank:2}, 
            Card{suit:Suit::Spades, rank:4}, 
            Card{suit:Suit::Spades, rank:6}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), None);
    }

    #[test]
    fn should_not_be_a_flush_2() {
        let cards = vec![
            Card{suit:Suit::Hearts, rank:5}, 
            Card{suit:Suit::Hearts, rank:2}, 
            Card{suit:Suit::Clubs, rank:5}, 
            Card{suit:Suit::Clubs, rank:2}, 
            Card{suit:Suit::Diamonds, rank:5}
        ];
        let hand = Hand::new(cards);
        assert_eq!(is_flush(hand.suits()), None);
    }


}
