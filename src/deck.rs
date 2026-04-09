use rand::{Rng, RngExt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Rank {
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
enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    const ALL_RANKS: [Rank; 13] = [
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

    const ALL_SUITS: [Suit; 4] = [Suit::Hearts, Suit::Spades, Suit::Clubs, Suit::Diamonds];

    fn from_rank_suit(rank: Rank, suit: Suit) -> Self {
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

        // let suit = match suit {
        //     0 => Suit::Hearts,
        //     1 => Suit::Spades,
        //     2 => Suit::Clubs,
        //     3 => Suit::Diamonds,
        //     _ => panic!(""),
        // };

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

#[derive(Clone, Copy, Hash)]
pub struct Deck(u64);

impl Deck {
    const FULL_MASK: u64 = (1u64 << 52) - 1;

    pub const fn new_full() -> Self {
        // First 52 bits set
        Self(Self::FULL_MASK)
    }

    pub const fn new_empty() -> Self {
        Self(0u64)
    }

    fn add_card(&self, card: &Card) -> Self {
        Self(self.0 | card.to_bit_mask())
    }

    fn remove_card(&self, card: &Card) -> Self {
        Self(self.0 & !card.to_bit_mask())
    }

    fn has_card(&self, card: &Card) -> bool {
        self.0 & card.to_bit_mask() > 0
    }

    fn remove_nth_card(&self, n: usize) -> (Self, Card) {
        todo!();
    }

    fn draw_random_card<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> (Self, Card) {
        let number_of_ones = self.0.count_ones();

        let k = rng.random_range(0usize..number_of_ones);

        self.remove_nth_card(k)
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
    fn draw_random_card_nonrandom() {
        let new_deck = Deck::new_full();
    }
}
