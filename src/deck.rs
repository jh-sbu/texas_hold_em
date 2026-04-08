#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn from_rank_suit(rank: Rank, suit: Suit) -> Self {
        todo!();
    }

    fn to_index(&self) -> usize {
        todo!();
    }

    fn to_bit_index(&self) -> u64 {
        todo!();
    }

    fn all_card_types() -> Vec<Card> {
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

        let mut card_types = Vec::<Card>::with_capacity(52);

        for rank in ALL_RANKS {
            for suit in ALL_SUITS {
                card_types.push(Card::from_rank_suit(rank, suit));
            }
        }

        card_types
    }
}

#[derive(Hash)]
pub struct Deck(u64);

impl Deck {
    pub fn new_full() -> Self {
        todo!();
    }

    pub fn new_empty() -> Self {
        Self(0)
    }

    fn add_card(&self, card: &Card) -> Self {
        Self(self.0 | card.to_bit_index())
    }

    fn remove_card(&self, card: &Card) -> Self {
        Self(self.0 & !card.to_bit_index())
    }

    fn has_card(&self, card: &Card) -> bool {
        self.0 & card.to_bit_index() > 0
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
}
