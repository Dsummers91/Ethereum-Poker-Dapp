
#[cfg(test)]
mod tests {
    use card::{Card, suit::Suit};

    #[test]
    fn should_sort_correctly() {
        let mut cards = vec![
            &Card{suit:Suit::Spades, rank: 13},
			&Card{suit:Suit::Diamonds, rank: 12},
			&Card{suit:Suit::Spades, rank: 14},
		];
		let correct_order = vec![
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 13},
			&Card{suit:Suit::Spades, rank: 12},
		];
		cards.sort();
		assert_eq!(cards, correct_order); 
	}

    #[test]
    fn should_sort_correctly_1() {
        let mut cards = vec![
            &Card{suit:Suit::Spades, rank: 13},
			&Card{suit:Suit::Diamonds, rank: 12},
			&Card{suit:Suit::Spades, rank: 14},
		];
		let correct_order = vec![
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 13},
			&Card{suit:Suit::Spades, rank: 12},
		];
		cards.sort();
		assert_eq!(cards, correct_order); 
	}

    #[test]
    fn should_sort_correctly_2() {
        let mut cards = vec![
            &Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 14},
            &Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Spades, rank: 2},
		];
		let correct_order = vec![
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 14},
			&Card{suit:Suit::Spades, rank: 2},
		];
		cards.sort();
		assert_eq!(cards, correct_order); 
	}

    #[test]
    fn should_sort_correctly_3() {
        let mut cards = vec![
            &Card{suit:Suit::Spades, rank: 14},
            &Card{suit:Suit::Spades, rank: 3},
			&Card{suit:Suit::Diamonds, rank: 2},
			&Card{suit:Suit::Spades, rank: 14},
		];
		let correct_order = vec![
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Spades, rank: 14},
			&Card{suit:Suit::Diamonds, rank: 3},
			&Card{suit:Suit::Spades, rank: 2},
		];
		cards.sort();
		assert_eq!(cards, correct_order); 
	}

    #[test]
    fn should_format_correctly() {
      let card = Card{suit:Suit::Spades, rank:14};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Ace of Spades");
    }


    #[test]
    fn should_format_correctly2() {
      let card = Card{suit:Suit::Clubs, rank:2};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "Deuce of Clubs");
    }

    #[test]
    fn should_format_correctly3() {
      let card = Card{suit:Suit::Diamonds, rank:5};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "5 of Diamonds");
    }


    #[test]
    fn should_format_correctly4() {
      let card = Card{suit:Suit::Hearts, rank:13};
      let formatted_card = format!("{}", card);
      assert_eq!(formatted_card, "King of Hearts");
    }

    #[test]
    fn deuce_should_have_rank_of_2() {
      let card = Card{suit:Suit::Hearts, rank:2};
      assert_eq!(card.rank(), vec![(2, &card)]);
    }

    #[test]
    fn ace_should_have_rank_if_1_or_14() {
      let card = Card{suit:Suit::Hearts, rank:14};
      assert_eq!(card.rank(), vec![(14, &card), (1, &card)]);
    }

    #[test]
    fn king_should_have_rank_if_13() {
      let card = Card{suit:Suit::Hearts, rank:13};
      assert_eq!(card.rank(), vec![(13, &card)]);
    }
}
