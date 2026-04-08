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
    fn to_index(&self) -> usize {
        todo!();
    }
}

pub struct Deck(u64);

impl Deck {
    fn add_card(&mut self) {
        todo!();
    }
}

pub type Hand = Deck;
