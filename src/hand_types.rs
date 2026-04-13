use crate::deck::Hand;

const HEARTS_MASK: u64 = 0b0000000000000000000000000000000000000000000000000001111111111111;
const SPADES_MASK: u64 = 0b0000000000000000000000000000000000000011111111111110000000000000;
const CLUBS_MASK: u64 = 0b0000000000000000000000000111111111111100000000000000000000000000;
const DIAMONDS_MASK: u64 = 0b0000000000001111111111111000000000000000000000000000000000000000;

const TWO_MASK: u64 = 0b0000000000000000000000001000000000000100000000000010000000000001;
const THREE_MASK: u64 = 0b0000000000000000000000010000000000001000000000000100000000000010;
const FOUR_MASK: u64 = 0b0000000000000000000000100000000000010000000000001000000000000100;
const FIVE_MASK: u64 = 0b0000000000000000000001000000000000100000000000010000000000001000;
const SIX_MASK: u64 = 0b0000000000000000000010000000000001000000000000100000000000010000;
const SEVEN_MASK: u64 = 0b0000000000000000000100000000000010000000000001000000000000100000;
const EIGHT_MASK: u64 = 0b0000000000000000001000000000000100000000000010000000000001000000;
const NINE_MASK: u64 = 0b0000000000000000010000000000001000000000000100000000000010000000;
const TEN_MASK: u64 = 0b0000000000000000100000000000010000000000001000000000000100000000;
const JACK_MASK: u64 = 0b0000000000000001000000000000100000000000010000000000001000000000;
const QUEEN_MASK: u64 = 0b0000000000000010000000000001000000000000100000000000010000000000;
const KING_MASK: u64 = 0b0000000000000100000000000010000000000001000000000000100000000000;
const ACE_MASK: u64 = 0b0000000000001000000000000100000000000010000000000001000000000000;

const ROYAL_MASK: u64 = ACE_MASK | KING_MASK | QUEEN_MASK | JACK_MASK | TEN_MASK;

const ALL_SUITS: [u64; 4] = [HEARTS_MASK, SPADES_MASK, CLUBS_MASK, DIAMONDS_MASK];

const ALL_RANKS: [u64; 13] = [
    TWO_MASK, THREE_MASK, FOUR_MASK, FIVE_MASK, SIX_MASK, SEVEN_MASK, EIGHT_MASK, NINE_MASK,
    TEN_MASK, JACK_MASK, QUEEN_MASK, KING_MASK, ACE_MASK,
];

// Two player cards and five dealer cards so >=, not ==
pub(crate) fn is_flush(hand: &Hand) -> bool {
    ((hand.0 & HEARTS_MASK).count_ones() >= 5)
        | ((hand.0 & SPADES_MASK).count_ones() >= 5)
        | ((hand.0 & CLUBS_MASK).count_ones() >= 5)
        | ((hand.0 & DIAMONDS_MASK).count_ones() >= 5)
}

pub(crate) fn is_royal_flush(hand: &Hand) -> bool {
    let mut hand = hand.clone();
    hand.0 = hand.0 & ROYAL_MASK;
    // let hand = hand.0 & ROYAL_MASK;

    ((hand.0 & ACE_MASK).count_ones() >= 1)
        && ((hand.0 & KING_MASK).count_ones() >= 1)
        && ((hand.0 & QUEEN_MASK).count_ones() >= 1)
        && ((hand.0 & JACK_MASK).count_ones() >= 1)
        && ((hand.0 & TEN_MASK).count_ones() >= 1)
        && is_flush(&hand)
}

pub(crate) fn is_straight(hand: &Hand) -> bool {
    let mut biggest_streak = 0;

    for rank in ALL_RANKS {
        if (hand.0 & rank).count_ones() > 0 {
            biggest_streak += 1;
        } else {
            biggest_streak = 0;
        }

        if biggest_streak == 5 {
            return true;
        }
    }

    false
}

pub(crate) fn is_straight_flush(hand: &Hand) -> bool {
    for suit in ALL_SUITS {
        let mut hand = hand.clone();
        hand.0 &= suit;

        if is_straight(&hand) {
            return true;
        }
    }

    false
}

pub(crate) fn is_four_of_a_kind(hand: &Hand) -> bool {
    for rank in ALL_RANKS {
        if (hand.0 & rank).count_ones() >= 4 {
            return true;
        }
    }

    false
}

pub(crate) fn is_full_house(hand: &Hand) -> bool {
    let mut found_three = false;
    let mut found_two = false;

    for rank in ALL_RANKS {
        match (hand.0 & rank).count_ones() {
            2 => {
                if found_three {
                    return true;
                } else {
                    found_two = true;
                }
            }
            3.. => {
                if found_two {
                    return true;
                } else {
                    found_three = true;
                }
            }
            _ => (),
        }
    }

    return false;
}

pub(crate) fn is_three_of_a_kind(hand: &Hand) -> bool {
    for rank in ALL_RANKS {
        if (hand.0 & rank).count_ones() >= 3 {
            return true;
        }
    }

    false
}

pub(crate) fn is_two_pair(hand: &Hand) -> bool {
    let mut found_first = false;

    for rank in ALL_RANKS {
        if (hand.0 & rank).count_ones() >= 2 {
            if found_first {
                return true;
            } else {
                found_first = true;
            }
        }
    }

    false
}

/// Even though this is called "is_one_pair" it
/// will match on two pair, full house, etc.
/// Should only be called after those options
/// have already been checked
pub(crate) fn is_one_pair(hand: &Hand) -> bool {
    for rank in ALL_RANKS {
        if (hand.0 & rank).count_ones() >= 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::{
        deck::{Card, Deck, Rank, Suit},
        hand_types::{
            CLUBS_MASK, DIAMONDS_MASK, HEARTS_MASK, SPADES_MASK, is_flush, is_four_of_a_kind,
            is_full_house, is_one_pair, is_royal_flush, is_straight, is_straight_flush,
            is_three_of_a_kind, is_two_pair,
        },
    };

    #[test]
    fn check_constants_1() {
        assert_eq!(HEARTS_MASK.count_ones(), 13);
    }

    #[test]
    fn check_constants_2() {
        assert_eq!(SPADES_MASK.count_ones(), 13);
    }

    #[test]
    fn check_constants_3() {
        assert_eq!(CLUBS_MASK.count_ones(), 13);
    }

    #[test]
    fn check_constants_4() {
        assert_eq!(DIAMONDS_MASK.count_ones(), 13);
    }

    #[test]
    fn check_constants_5() {
        assert_eq!(
            (HEARTS_MASK | SPADES_MASK | CLUBS_MASK | DIAMONDS_MASK).count_ones(),
            52
        );
    }

    #[test]
    fn is_flush_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts));

        assert!(is_flush(&deck));
    }

    #[test]
    fn is_flush_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Hearts));

        assert!(!is_flush(&deck));
    }

    #[test]
    fn is_flush_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts));

        assert!(!is_flush(&deck));
    }

    #[test]
    fn is_royal_flush_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Spades));

        assert!(is_royal_flush(&deck));
    }

    #[test]
    fn is_royal_flush_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts));

        assert!(!is_royal_flush(&deck));
    }

    #[test]
    /// Is a flush but not royal flush because ten is off suit
    fn is_royal_flush_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Nine, Suit::Spades));

        assert!(!is_royal_flush(&deck));
    }

    #[test]
    fn is_straight_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Hearts));

        assert!(is_straight(&deck));
    }

    #[test]
    fn is_straight_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Hearts));

        assert!(!is_straight(&deck));
    }

    #[test]
    fn is_one_pair_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(is_one_pair(&deck));
    }

    #[test]
    fn is_one_pair_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_one_pair(&deck));
    }

    #[test]
    fn is_two_pair_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(is_two_pair(&deck));
    }

    #[test]
    fn is_two_pair_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_two_pair(&deck));
    }

    #[test]
    fn is_full_house_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs));

        assert!(is_full_house(&deck));
    }

    #[test]
    fn is_full_house_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_full_house(&deck));
    }

    #[test]
    fn is_three_of_a_kind_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(is_three_of_a_kind(&deck));
    }

    #[test]
    fn is_three_of_a_kind_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_three_of_a_kind(&deck));
    }

    #[test]
    fn is_four_of_a_kind_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs));

        assert!(is_four_of_a_kind(&deck));
    }

    #[test]
    fn is_four_of_a_kind_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_four_of_a_kind(&deck));
    }

    #[test]
    fn is_straight_flush_1() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(is_straight_flush(&deck));
    }

    #[test]
    fn is_straight_flush_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert!(!is_straight_flush(&deck));
    }
}
