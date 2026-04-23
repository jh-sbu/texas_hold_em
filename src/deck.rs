use rand::RngExt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub(crate) enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub(crate) const ALL_RANKS: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    pub(crate) const ALL_SUITS: [Suit; 4] =
        [Suit::Hearts, Suit::Spades, Suit::Clubs, Suit::Diamonds];

    pub(crate) fn from_rank_suit(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    /// Index -> Card, inverse of to_index
    /// Cards are ordered by suit, then rank,
    /// aces high
    fn from_index(index: usize) -> Self {
        assert!(
            index < 52,
            "You tried to get the card whose index is {}, but the highest index is 51 (52 cards but index starts at 0)",
            index
        );
        let suit = index / 13;
        let rank = index % 13;

        let suit = Self::ALL_SUITS[suit];

        let rank = Self::ALL_RANKS[rank];

        Card::from_rank_suit(rank, suit)
    }

    fn to_index(&self) -> usize {
        (self.suit as usize) * 13 + (self.rank as usize)
    }

    fn to_bit_mask(&self) -> u64 {
        1u64 << ((self.suit as u8) * 13 + (self.rank as u8))
    }

    fn all_card_types() -> Vec<Card> {
        let mut card_types = Vec::<Card>::with_capacity(52);

        for rank in Self::ALL_RANKS {
            for suit in Self::ALL_SUITS {
                card_types.push(Card::from_rank_suit(rank, suit));
            }
        }

        card_types
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Deck(pub(crate) u64);

impl Deck {
    const FULL_MASK: u64 = (1u64 << 52) - 1;

    pub const fn new_full() -> Self {
        // First 52 bits set
        Self(Self::FULL_MASK)
    }

    pub const fn new_empty() -> Self {
        Self(0u64)
    }

    pub(crate) fn add_card(&self, card: &Card) -> Self {
        debug_assert_eq!(
            self.0 & card.to_bit_mask(),
            0,
            "Tried to add card {:?} to a deck it was already present in",
            card
        );
        Self(self.0 | card.to_bit_mask())
    }

    pub(crate) fn remove_card(&self, card: &Card) -> Self {
        Self(self.0 & !card.to_bit_mask())
    }

    fn has_card(&self, card: &Card) -> bool {
        self.0 & card.to_bit_mask() > 0
    }

    fn remove_nth_card(&self, n: u32) -> (Self, Card) {
        let mut i = 0;
        let mut count = 0;
        loop {
            // Check the i-th bit
            if self.0 & (1u64 << i) > 0 {
                if count == n {
                    let card = Card::from_index(i);
                    return (self.remove_card(&card), card);
                } else {
                    count += 1;
                }
            }

            i += 1;
        }
    }

    pub(crate) fn draw_random_card<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> (Self, Card) {
        let number_of_ones = self.0.count_ones();

        let k = rng.random_range(0..number_of_ones);

        self.remove_nth_card(k)
    }

    pub(crate) fn draw_card_to_hand<R: rand::Rng + ?Sized>(
        &self,
        hand: &Hand,
        rng: &mut R,
    ) -> (Self, Hand) {
        let (new_deck, new_card) = self.draw_random_card(rng);

        (new_deck, Deck(hand.0 | new_card.to_bit_mask()))
    }
}

pub type Hand = Deck;

#[cfg(test)]
mod test {
    use crate::deck::{Card, Deck, Hand, Rank, Suit};

    #[test]
    fn new_empty_1() {
        let new_deck = Deck::new_empty();

        for card in Card::all_card_types() {
            assert!(!new_deck.has_card(&card));
        }
    }

    #[test]
    fn new_empty_2() {
        let new_hand = Hand::new_empty();

        for card in Card::all_card_types() {
            assert!(!new_hand.has_card(&card));
        }
    }

    #[test]
    fn new_full_1() {
        let new_deck = Deck::new_full();

        for card in Card::all_card_types() {
            assert!(new_deck.has_card(&card));
        }
    }

    #[test]
    fn new_full_2() {
        let new_hand = Hand::new_full();

        for card in Card::all_card_types() {
            assert!(new_hand.has_card(&card));
        }
    }

    #[test]
    fn add_card_1() {
        let new_deck = Deck::new_empty();

        let new_deck = new_deck.add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts));

        assert!(new_deck.has_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts)))
    }

    #[test]
    fn add_card_2() {
        let new_deck = Hand::new_empty();

        let new_deck = new_deck.add_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts));

        assert!(new_deck.has_card(&Card::from_rank_suit(Rank::Ten, Suit::Hearts)))
    }

    #[test]
    fn remove_card_1() {
        let new_deck = Deck::new_full();

        let card = Card::from_rank_suit(Rank::Eight, Suit::Clubs);

        assert!(new_deck.has_card(&card));

        let new_deck = new_deck.remove_card(&card);

        assert!(!new_deck.has_card(&card));
    }

    #[test]
    fn remove_card_2() {
        let new_deck = Deck::new_empty();

        let card = Card::from_rank_suit(Rank::Queen, Suit::Diamonds);

        assert!(!new_deck.has_card(&card));

        let new_deck = new_deck.add_card(&card);

        assert!(new_deck.has_card(&card));

        let new_deck = new_deck.remove_card(&card);

        assert!(!new_deck.has_card(&card));
    }

    #[test]
    fn draw_random_card_nonrandom_1() {
        let new_deck = Deck::new_full();

        let (deck, card) = new_deck.remove_nth_card(0);

        let correct_deck = Deck(Deck::FULL_MASK - 1);

        assert_eq!(deck, correct_deck);
        assert_eq!(card, Card::from_rank_suit(Rank::Two, Suit::Hearts));
    }

    #[test]
    fn draw_random_card_nonrandom_2() {
        let new_deck = Deck::new_full();

        let (deck, card) = new_deck.remove_nth_card(3);

        let correct_deck = Deck(Deck::FULL_MASK - 8);

        assert_eq!(deck, correct_deck);
        assert_eq!(card, Card::from_rank_suit(Rank::Five, Suit::Hearts));
    }

    #[test]
    fn draw_random_card_nonrandom_3() {
        let new_deck = Deck::new_full();

        let (deck, card) = new_deck.remove_nth_card(51);

        let correct_deck = Deck(Deck::FULL_MASK - (1u64 << 51));

        assert_eq!(deck, correct_deck);
        assert_eq!(card, Card::from_rank_suit(Rank::Ace, Suit::Diamonds));
    }

    #[test]
    fn draw_random_card_random_1() {
        let new_deck = Deck::new_full();

        let mut rng = rand::rng();

        let (deck, _) = new_deck.draw_random_card(&mut rng);
        assert_eq!(deck.0.count_ones(), 51);
    }

    #[test]
    fn draw_random_card_random_multiple_1() {
        let mut rng = rand::rng();

        let new_deck = Deck::new_full();

        let (deck, _) = new_deck.draw_random_card(&mut rng);
        let (deck, _) = deck.draw_random_card(&mut rng);
        let (deck, _) = deck.draw_random_card(&mut rng);
        let (deck, _) = deck.draw_random_card(&mut rng);
        let (deck, _) = deck.draw_random_card(&mut rng);
        assert_eq!(deck.0.count_ones(), 47);
    }

    #[test]
    fn correct_deck_card_count_1() {
        let new_deck = Deck::new_full();

        assert_eq!(new_deck.0.count_ones(), 52);
    }

    #[test]
    fn correct_deck_card_count_2() {
        let new_deck = Deck::new_empty();

        assert_eq!(new_deck.0.count_ones(), 0);
    }

    #[test]
    fn correct_deck_card_count_3() {
        let (new_deck, _) = Deck::new_full().remove_nth_card(0);

        assert_eq!(new_deck.0.count_ones(), 51);
    }

    #[test]
    fn correct_deck_card_count_4() {
        let new_deck = Deck::new_empty().add_card(&Card::from_rank_suit(Rank::Six, Suit::Spades));

        assert_eq!(new_deck.0.count_ones(), 1);
    }

    #[test]
    fn from_index_1() {
        let i = 0;
        let c = Card::from_index(i);

        assert_eq!(c.rank, Rank::Two);
        assert_eq!(c.suit, Suit::Hearts);
    }

    #[test]
    fn from_index_2() {
        let i = 14;
        let c = Card::from_index(i);

        assert_eq!(c.rank, Rank::Three);
        assert_eq!(c.suit, Suit::Spades);
    }

    #[test]
    fn from_index_3() {
        let i = 28;
        let c = Card::from_index(i);

        assert_eq!(c.rank, Rank::Four);
        assert_eq!(c.suit, Suit::Clubs);
    }

    #[test]
    fn from_index_4() {
        let i = 43;
        let c = Card::from_index(i);

        assert_eq!(c.rank, Rank::Six);
        assert_eq!(c.suit, Suit::Diamonds);
    }

    #[test]
    fn from_index_5() {
        let i = 51;
        let c = Card::from_index(i);

        assert_eq!(c.rank, Rank::Ace);
        assert_eq!(c.suit, Suit::Diamonds);
    }
}
