use crate::deck::{Card, Hand, Rank};

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

const ALL_SUIT_MASKS: [u64; 4] = [HEARTS_MASK, SPADES_MASK, CLUBS_MASK, DIAMONDS_MASK];

const ALL_RANK_MASKS: [u64; 13] = [
    TWO_MASK, THREE_MASK, FOUR_MASK, FIVE_MASK, SIX_MASK, SEVEN_MASK, EIGHT_MASK, NINE_MASK,
    TEN_MASK, JACK_MASK, QUEEN_MASK, KING_MASK, ACE_MASK,
];

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum HandType {
    RoyalFlush,
    StraightFlush(Rank),
    FourOfAKind(Rank),
    FullHouse(Rank, Rank),
    Flush,
    Straight(Rank),
    ThreeOfAKind(Rank),
    TwoPair(Rank, Rank),
    OnePair(Rank),
    HighCard(Rank),
}

// Two player cards and five dealer cards so >=, not ==
// Return the full hand since flush ties are not uniquely
// broken by one or two ranks, need all of them
pub(crate) fn is_flush(hand: &Hand) -> Option<HandType> {
    for suit_mask in ALL_SUIT_MASKS {
        let mut hand = hand.clone();
        hand.0 &= suit_mask;

        if hand.0.count_ones() >= 5 {
            return Some(HandType::Flush);
        }
    }

    None
}

pub(crate) fn is_royal_flush(hand: &Hand) -> Option<HandType> {
    if let Some(HandType::StraightFlush(rank)) = is_straight_flush(hand)
        && rank == Rank::Ace
    {
        Some(HandType::RoyalFlush)
    } else {
        None
    }
}

pub(crate) fn is_straight(hand: &Hand) -> Option<HandType> {
    // Special handling for ace low
    let mut biggest_streak = if (hand.0 & ACE_MASK) > 0 { 1 } else { 0 };

    for (i, rank_mask) in ALL_RANK_MASKS.iter().enumerate() {
        if (hand.0 & rank_mask).count_ones() > 0 {
            biggest_streak += 1;
        } else if biggest_streak >= 5 {
            // Fine to subtract here since biggest_streak >= 5 => i > 0 and i - 1 <= .len() - 1
            return Some(HandType::Straight(Card::ALL_RANKS[i - 1]));
        } else {
            biggest_streak = 0;
        }
    }

    if biggest_streak >= 5 {
        return Some(HandType::Straight(Rank::Ace));
    }

    None
}

pub(crate) fn is_straight_flush(hand: &Hand) -> Option<HandType> {
    for suit_mask in ALL_SUIT_MASKS {
        let mut hand = hand.clone();
        hand.0 &= suit_mask;

        if let Some(HandType::Straight(rank)) = is_straight(&hand) {
            return Some(HandType::StraightFlush(rank));
        }
    }

    None
}

/// 5 dealer + 2 player cards -> at most 1 4 of a kind
pub(crate) fn is_four_of_a_kind(hand: &Hand) -> Option<HandType> {
    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS) {
        if (hand.0 & rank_mask).count_ones() >= 4 {
            return Some(HandType::FourOfAKind(rank));
        }
    }

    None
}

pub(crate) fn is_full_house(hand: &Hand) -> Option<HandType> {
    let mut found_three = None;
    let mut found_two = None;

    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS).rev() {
        match (hand.0 & rank_mask).count_ones() {
            2 => {
                if let Some(three_rank) = found_three {
                    return Some(HandType::FullHouse(three_rank, rank));
                } else {
                    found_two = Some(rank);
                }
            }
            3.. => {
                if let Some(two_rank) = found_two {
                    return Some(HandType::FullHouse(rank, two_rank));
                } else {
                    found_three = Some(rank);
                }
            }
            _ => (),
        }
    }

    None
}

/// Will evaluate to Some(ThreeOfAKind) if you don't test for
/// full house first (also test for four of a kind, in case it
/// exists for a different rank!)
pub(crate) fn is_three_of_a_kind(hand: &Hand) -> Option<HandType> {
    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS).rev() {
        if (hand.0 & rank_mask).count_ones() == 3 {
            return Some(HandType::ThreeOfAKind(rank));
        }
    }

    None
}

/// Will evaluate to Some(TwoPair) if you don't test for full house
/// or three of a kind, etc. first
pub(crate) fn is_two_pair(hand: &Hand) -> Option<HandType> {
    let mut found_higher_pair = None;

    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS).rev() {
        if (hand.0 & rank_mask).count_ones() == 2 {
            if let Some(higher_rank) = found_higher_pair {
                return Some(HandType::TwoPair(higher_rank, rank));
            } else {
                found_higher_pair = Some(rank);
            }
        }
    }

    None
}

/// Test for higher options first!
pub(crate) fn is_one_pair(hand: &Hand) -> Option<HandType> {
    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS).rev() {
        if (hand.0 & rank_mask).count_ones() == 2 {
            return Some(HandType::OnePair(rank));
        }
    }

    None
}

fn highest_card(hand: &Hand) -> HandType {
    assert_ne!(
        hand.0.count_ones(),
        0,
        "You can't find the highest card in a hand that has no cards"
    );

    for (rank_mask, rank) in ALL_RANK_MASKS.iter().zip(Card::ALL_RANKS).rev() {
        if hand.0 & rank_mask > 0 {
            return HandType::HighCard(rank);
        }
    }

    unreachable!()
}

fn compare_high(hand_1: &Hand, hand_2: &Hand) -> u8 {
    todo!();
}

/// Todo! Do this correctly instead of incorrectly
pub(crate) fn highest_hand(hand: &Hand) -> HandType {
    [
        is_royal_flush,
        is_straight_flush,
        is_four_of_a_kind,
        is_full_house,
        is_flush,
        is_straight,
        is_three_of_a_kind,
        is_two_pair,
        is_one_pair,
    ]
    .into_iter()
    .find_map(|func| func(hand))
    .unwrap_or_else(|| highest_card(hand))
}

#[cfg(test)]
mod test {
    use crate::{
        deck::{Card, Deck, Rank, Suit},
        hand_types::{
            ACE_MASK, CLUBS_MASK, DIAMONDS_MASK, EIGHT_MASK, FIVE_MASK, FOUR_MASK, HEARTS_MASK,
            HandType, JACK_MASK, KING_MASK, NINE_MASK, QUEEN_MASK, SEVEN_MASK, SIX_MASK,
            SPADES_MASK, TEN_MASK, THREE_MASK, TWO_MASK, highest_card, highest_hand, is_flush,
            is_four_of_a_kind, is_full_house, is_one_pair, is_royal_flush, is_straight,
            is_straight_flush, is_three_of_a_kind, is_two_pair,
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
    fn check_constants_6() {
        assert_eq!(TWO_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_7() {
        assert_eq!(THREE_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_8() {
        assert_eq!(FOUR_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_9() {
        assert_eq!(FIVE_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_10() {
        assert_eq!(SIX_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_11() {
        assert_eq!(SEVEN_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_12() {
        assert_eq!(EIGHT_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_13() {
        assert_eq!(NINE_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_14() {
        assert_eq!(TEN_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_15() {
        assert_eq!(JACK_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_16() {
        assert_eq!(QUEEN_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_17() {
        assert_eq!(KING_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_18() {
        assert_eq!(ACE_MASK.count_ones(), 4);
    }

    #[test]
    fn check_constants_19() {
        assert_eq!(
            (TWO_MASK
                | THREE_MASK
                | FOUR_MASK
                | FIVE_MASK
                | SIX_MASK
                | SEVEN_MASK
                | EIGHT_MASK
                | NINE_MASK
                | TEN_MASK
                | JACK_MASK
                | QUEEN_MASK
                | KING_MASK
                | ACE_MASK)
                .count_ones(),
            52
        )
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

        assert_eq!(is_flush(&deck), Some(HandType::Flush));
    }

    #[test]
    fn is_flush_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Hearts));

        assert_eq!(is_flush(&deck), None);
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

        assert_eq!(is_flush(&deck), None);
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

        assert_eq!(is_royal_flush(&deck), Some(HandType::RoyalFlush));
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

        assert_eq!(is_royal_flush(&deck), None);
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

        assert_eq!(is_royal_flush(&deck), None);
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

        assert_eq!(is_straight(&deck), Some(HandType::Straight(Rank::Six)));
    }

    #[test]
    fn is_straight_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Nine, Suit::Diamonds));

        assert_eq!(is_straight(&deck), Some(HandType::Straight(Rank::Five)));
    }

    #[test]
    fn is_straight_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Diamonds));

        assert_eq!(is_straight(&deck), Some(HandType::Straight(Rank::Ace)));
    }

    #[test]
    fn is_straight_4() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Hearts));

        assert_eq!(is_straight(&deck), None);
    }

    #[test]
    fn is_straight_5() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Hearts));

        assert_eq!(is_straight(&deck), Some(HandType::Straight(Rank::Seven)));
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

        assert_eq!(is_one_pair(&deck), Some(HandType::OnePair(Rank::Two)));
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

        assert_eq!(is_one_pair(&deck), None);
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

        assert_eq!(
            is_two_pair(&deck),
            Some(HandType::TwoPair(Rank::Four, Rank::Two))
        );
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

        assert_eq!(is_two_pair(&deck), None);
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

        assert_eq!(
            is_full_house(&deck),
            Some(HandType::FullHouse(Rank::Eight, Rank::Two))
        );
    }

    #[test]
    fn is_full_house_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades));

        assert_eq!(
            is_full_house(&deck),
            Some(HandType::FullHouse(Rank::Eight, Rank::Two))
        );
    }

    #[test]
    fn is_full_house_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_full_house(&deck), None);
    }

    #[test]
    fn is_full_house_4() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_full_house(&deck), None);
    }

    #[test]
    fn is_full_house_5() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_full_house(&deck), None);
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

        assert_eq!(
            is_three_of_a_kind(&deck),
            Some(HandType::ThreeOfAKind(Rank::Four))
        );
    }

    #[test]
    fn is_three_of_a_kind_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_three_of_a_kind(&deck), None);
    }

    #[test]
    fn is_three_of_a_kind_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_three_of_a_kind(&deck), None);
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

        assert_eq!(
            is_four_of_a_kind(&deck),
            Some(HandType::FourOfAKind(Rank::Four))
        );
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

        assert_eq!(is_four_of_a_kind(&deck), None);
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

        assert_eq!(
            is_straight_flush(&deck),
            Some(HandType::StraightFlush(Rank::Six))
        );
    }

    #[test]
    fn is_straight_flush_2() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(
            is_straight_flush(&deck),
            Some(HandType::StraightFlush(Rank::Five))
        );
    }

    #[test]
    fn is_straight_flush_3() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Clubs));

        assert_eq!(
            is_straight_flush(&deck),
            Some(HandType::StraightFlush(Rank::Ace))
        );
    }

    #[test]
    fn is_straight_flush_4() {
        let deck = Deck::new_empty();

        let deck = deck
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs));

        assert_eq!(is_straight_flush(&deck), None);
    }

    #[test]
    fn eval_high_card_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::HighCard(Rank::King));
    }

    #[test]
    fn eval_high_card_2() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::HighCard(Rank::Ace));
    }

    #[test]
    fn eval_one_pair_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::OnePair(Rank::Seven));
    }

    #[test]
    fn eval_one_pair_2() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::OnePair(Rank::Jack));
    }

    #[test]
    fn eval_two_pair_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(
            highest_hand(&deck),
            HandType::TwoPair(Rank::Jack, Rank::Seven)
        );
    }

    #[test]
    fn highest_card_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_card(&deck), HandType::HighCard(Rank::King));
    }

    #[test]
    fn highest_card_2() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_card(&deck), HandType::HighCard(Rank::Ace));
    }

    #[test]
    fn highest_card_3() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Jack, Suit::Hearts));

        assert_eq!(highest_card(&deck), HandType::HighCard(Rank::Jack));
    }

    #[test]
    fn highest_card_4() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(highest_card(&deck), HandType::HighCard(Rank::Ten));
    }

    #[test]
    fn highest_card_5() {
        let deck = Deck::new_empty().add_card(&Card::from_rank_suit(Rank::Two, Suit::Hearts));

        assert_eq!(highest_card(&deck), HandType::HighCard(Rank::Two));
    }

    #[test]
    fn highest_hand_straight_flush_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ace, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs));

        assert_eq!(highest_hand(&deck), HandType::StraightFlush(Rank::Eight));
    }

    #[test]
    fn highest_hand_straight_flush_2() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs));

        assert_eq!(highest_hand(&deck), HandType::StraightFlush(Rank::Eight));
    }

    #[test]
    fn highest_hand_four_of_a_kind_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::FourOfAKind(Rank::Four));
    }

    #[test]
    fn highest_hand_full_house_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(
            highest_hand(&deck),
            HandType::FullHouse(Rank::Four, Rank::Seven)
        );
    }

    #[test]
    fn highest_hand_flush_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Queen, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::King, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Clubs));

        assert_eq!(highest_hand(&deck), HandType::Flush);
    }

    #[test]
    fn highest_hand_straight_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Five, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Six, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::Straight(Rank::Eight));
    }

    #[test]
    fn highest_hand_three_of_a_kind_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::ThreeOfAKind(Rank::Four));
    }

    #[test]
    fn highest_hand_two_pair_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts));

        assert_eq!(
            highest_hand(&deck),
            HandType::TwoPair(Rank::Ten, Rank::Eight)
        );
    }

    #[test]
    fn highest_hand_two_pair_2() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Hearts));

        assert_eq!(
            highest_hand(&deck),
            HandType::TwoPair(Rank::Seven, Rank::Four)
        );
    }

    #[test]
    fn highest_hand_one_pair_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Nine, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::OnePair(Rank::Eight));
    }

    #[test]
    fn highest_hand_high_card_1() {
        let deck = Deck::new_empty()
            .add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts))
            .add_card(&Card::from_rank_suit(Rank::Two, Suit::Spades))
            .add_card(&Card::from_rank_suit(Rank::Eight, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Nine, Suit::Diamonds))
            .add_card(&Card::from_rank_suit(Rank::Four, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Three, Suit::Clubs))
            .add_card(&Card::from_rank_suit(Rank::Seven, Suit::Hearts));

        assert_eq!(highest_hand(&deck), HandType::HighCard(Rank::Ten));
    }
}
