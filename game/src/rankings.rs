use card::{Card, Suit, Cards};
use hand::{Hand};
use std::collections::HashMap;

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
    if let Some(x) = is_pair(_card.cards.to_vec()) {
        return Some(Ranks::OnePair)
    }
    None
}

// Should return flush
fn is_flush<'a, 'b>(mut h: &mut Hand<'a, 'b>) -> Option<Rank<'a>> {
    let suits = h.suits();
    let hand = h.cards.to_vec();
    if let Some(count_hearts) = suits.get(&Suit::Hearts) {
        if (count_hearts >= &5) {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(hand, Suit::Hearts)})
        }
    }
    if let Some(count_diamonds) = suits.get(&Suit::Diamonds) {
        if (count_diamonds >= &5) {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(hand, Suit::Diamonds)})
        }
    }
    if let Some(count_spades) = suits.get(&Suit::Spades) {
        if (count_spades >= &5) {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(hand, Suit::Spades)})
        }
    }
    if let Some(count_clubs) = suits.get(&Suit::Clubs) {
        if (count_clubs >= &5) {
            return Some(Rank{rank: Ranks::Flush, cards: get_flush_hand(hand, Suit::Clubs)})
        }
    }
    None
}

fn get_flush_cards(mut cards: Vec<&Card>, suit: Suit) -> Vec<&Card> {
    cards.iter().filter(|card| card.suit == suit).map(|card| *card).collect()
}

fn get_flush_hand(mut cards: Vec<&Card>, suit: Suit) -> Vec<&Card> {
    cards.sort();
    cards.iter().filter(|card| card.suit == suit).take(5).map(|card| *card).collect()
}


fn is_straight_flush<'a, 'b>(mut c: Vec<&Card>) -> Option<u8> {
    let cards = Cards(c.clone());
    let suits = cards.suits();
    if let Some(count_hearts) = suits.get(&Suit::Hearts) {
        if (count_hearts >= &5) {
            let result = get_flush_cards(c, Suit::Hearts);
            return is_straight(result)
        }
    }
    if let Some(count_diamonds) = suits.get(&Suit::Diamonds) {
        if (count_diamonds >= &5) {
            return is_straight(get_flush_cards(c, Suit::Diamonds))
        }
    }
    if let Some(count_spades) = suits.get(&Suit::Spades) {
        if (count_spades >= &5) {
            return is_straight(get_flush_cards(c, Suit::Spades))
        }
    }
    if let Some(count_clubs) = suits.get(&Suit::Clubs) {
        if (count_clubs >= &5) {
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

fn is_pair(cards: Vec<&Card>) -> Option<u8> {
    let ranks = Cards(cards).ranks();
    for rank in ranks.windows(2) {
        if rank[0] == rank[1] {
            return Some(rank[0]);
        }
    }
    None
}

fn is_two_pair(mut cards: Vec<&Card>) -> Option<[u8; 2]> {
    let mut leftover_cards = cards.clone();
    if let Some(x) = is_pair(cards) {
        leftover_cards = leftover_cards.iter().filter(|r| r.rank != x).cloned().collect();
        if let Some(y) = is_pair(leftover_cards) {
            return Some([x, y])
        }
    } 
    None
}

fn is_full_house(mut ranks: Vec<u8>) -> Option<[u8; 2]> {
    let mut leftover_cards = ranks.clone();
    if let Some(x) = is_three_of_a_kind(ranks) {
        leftover_cards = leftover_cards.iter().filter(|r| **r != x && **r > 1).cloned().collect();
        //if let Some(y) = is_pair(leftover_cards.to_vec()) {
        //    return Some([x, y])
        //}
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
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_pair(hand.cards.to_vec()), Some(2));
    }

    #[test]
    fn should_not_be_a_fullhouse() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:3} 
        ];
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_full_house(hand.ranks()), None);
    }

    #[test]
    fn should_be_two_pair_5_and_10() {
        let mut cards = vec![
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
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3} 
        ];
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_full_house(hand.ranks()), Some([3, 2]));
    }

    #[test]
    fn should_be_a_three_of_a_kind_queens() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:6}, 
            &Card{suit:Suit::Hearts, rank:12}, 
            &Card{suit:Suit::Hearts, rank:7}, 
            &Card{suit:Suit::Hearts, rank:12}
        ];
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_three_of_a_kind(hand.ranks()), Some(12));
    }

    #[test]
    fn should_be_a_four_of_a_kind_9() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:9}, 
            &Card{suit:Suit::Hearts, rank:2} 
        ];
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_four_of_a_kind(hand.ranks()), Some(9));
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
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_straight(hand.cards.to_vec()), Some(7));
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
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_straight(hand.cards.to_vec()), Some(14));
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
        let mut hand = Hand::new(&mut cards);
        assert_eq!(is_straight(hand.cards.to_vec()), Some(5));
    }

    #[test]
    fn should_be_a_hearts_flush() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        let mut flush_result = is_flush(&mut hand).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
        let mut iter = flush_result.cards.iter();
        assert_eq!(&hand.cards[0], iter.next().unwrap());
        assert_eq!(&hand.cards[1], iter.next().unwrap());
        assert_eq!(&hand.cards[2], iter.next().unwrap());
        assert_eq!(&hand.cards[3], iter.next().unwrap());
        assert_eq!(&hand.cards[4], iter.next().unwrap());
    }

    #[test]
    fn should_return_highest_five_flush() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:4}, 
            &Card{suit:Suit::Hearts, rank:8}, 
            &Card{suit:Suit::Hearts, rank:6}
        ];
        let flush = get_flush_cards(cards.to_vec(), Suit::Hearts);
        let mut high_flush_cards = [
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Hearts, rank:8}, 
            &Card{suit:Suit::Hearts, rank:6},
            &Card{suit:Suit::Hearts, rank:4}, 
        ];
        assert_eq!(flush, high_flush_cards);
    }

    #[test]
    fn should_be_a_straight_flush_five_high() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Hearts, rank:3}, 
            &Card{suit:Suit::Hearts, rank:14}, 
            &Card{suit:Suit::Hearts, rank:4}
        ];
        let flush_result = is_straight_flush(cards.to_vec());
        assert_eq!(flush_result, Some(5));
    }

    #[test]
    fn should_be_a_straight_flush_ace_high() {
        let mut cards = [
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
        let mut cards = [
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
        let mut cards = [
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
        let mut cards = [
            &Card{suit:Suit::Diamonds, rank:14}, 
            &Card{suit:Suit::Diamonds, rank:10}, 
            &Card{suit:Suit::Diamonds, rank:2}, 
            &Card{suit:Suit::Diamonds, rank:4}, 
            &Card{suit:Suit::Diamonds, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        let flush_result = is_flush(&mut hand).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_be_a_clubs_flush() {
        let mut cards = [
            &Card{suit:Suit::Clubs, rank:14}, 
            &Card{suit:Suit::Clubs, rank:10}, 
            &Card{suit:Suit::Clubs, rank:2}, 
            &Card{suit:Suit::Clubs, rank:4}, 
            &Card{suit:Suit::Clubs, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        let flush_result = is_flush(&mut hand).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_be_a_spades_flush() {
        let mut cards = [
            &Card{suit:Suit::Spades, rank:14}, 
            &Card{suit:Suit::Spades, rank:10}, 
            &Card{suit:Suit::Spades, rank:2}, 
            &Card{suit:Suit::Spades, rank:4}, 
            &Card{suit:Suit::Spades, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        let flush_result = is_flush(&mut hand).unwrap();
        assert_eq!(flush_result.rank, Ranks::Flush);
    }

    #[test]
    fn should_not_be_a_flush_1() {
        let mut cards = [
            &Card{suit:Suit::Spades, rank:14}, 
            &Card{suit:Suit::Hearts, rank:10}, 
            &Card{suit:Suit::Spades, rank:2}, 
            &Card{suit:Suit::Spades, rank:4}, 
            &Card{suit:Suit::Spades, rank:6}
        ];
        let mut hand = Hand::new(&mut cards);
        let flush_result = is_flush(&mut hand);
        assert_eq!(flush_result, None);
    }

    #[test]
    fn should_not_be_a_flush_2() {
        let mut cards = [
            &Card{suit:Suit::Hearts, rank:5}, 
            &Card{suit:Suit::Hearts, rank:2}, 
            &Card{suit:Suit::Clubs, rank:5}, 
            &Card{suit:Suit::Clubs, rank:2}, 
            &Card{suit:Suit::Diamonds, rank:5}
        ];
        let mut hand = Hand::new(&mut cards);
        let flush_result = is_flush(&mut hand);
        assert_eq!(flush_result, None);
    }


}
