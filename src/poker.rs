use crate::{
    deck::{Deck, Hand},
    player::Player,
};

const MIN_BET_TEMP: u64 = 80;

#[derive(Debug, Hash, Eq, PartialEq)]
enum PokerState {
    // No real use for MCTS to simulate before players get their hands
    // since I'm not accounting for straddle
    // NewGame,
    PreFlopRound,
    Flop,
    FlopRound,
    Turn,
    TurnRound,
    River,
    RiverRound,
    GameOver(Vec<Player>),
}

#[derive(Hash)]
pub struct PokerGame {
    players: Vec<Player>,
    dealer: usize,
    dealer_hand: Hand,
    deck: Deck,
    state: PokerState,
    min_bet: u64,
}

impl PokerGame {
    pub fn from_hands_money_and_dealer(hands: Vec<Hand>, money: u64, dealer: usize) -> PokerGame {
        let players = hands
            .iter()
            .map(|hand| Player::from_money_and_hand(money, hand))
            .collect();

        let mut deck = Deck::new_full();

        let _ = hands.iter().map(|hand| deck.0 &= !hand.0);

        PokerGame {
            players,
            dealer,
            dealer_hand: Hand::new_empty(),
            deck,
            state: PokerState::RiverRound,
            min_bet: MIN_BET_TEMP,
        }
    }

    // Move three cards from the deck to the dealer hand
    pub fn flop(self) -> PokerGame {
        // let new_cards = Hand::new_empty();

        let mut rng = rand::rng();

        let (deck, dealer_hand) =
            (0..3).fold((self.deck, Hand::new_empty()), |(deck, dealer_hand), _| {
                let (deck, new_dealer_card) = deck.draw_random_card(&mut rng);
                let dealer_hand = dealer_hand.add_card(&new_dealer_card);

                (deck, dealer_hand)
            });

        // let dealer_hand = Hand::new_empty();
        //
        // let (deck, new_dealer_card) = self.deck.draw_random_card(&mut rng);
        // let dealer_hand = dealer_hand.add_card(&new_dealer_card);
        //
        // let (deck, new_dealer_card) = deck.draw_random_card(&mut rng);
        // let dealer_hand = dealer_hand.add_card(&new_dealer_card);
        //
        // let (deck, new_dealer_card) = deck.draw_random_card(&mut rng);
        // let dealer_hand = dealer_hand.add_card(&new_dealer_card);

        PokerGame {
            players: self.players,
            dealer: self.dealer,
            dealer_hand,
            deck,
            state: PokerState::FlopRound,
            min_bet: self.min_bet,
        }
    }

    pub fn turn(self) -> PokerGame {
        let mut rng = rand::rng();

        let (deck, new_dealer_card) = self.deck.draw_random_card(&mut rng);
        let dealer_hand = self.dealer_hand.add_card(&new_dealer_card);

        PokerGame {
            players: self.players,
            dealer: self.dealer,
            dealer_hand,
            deck,
            state: PokerState::TurnRound,
            min_bet: self.min_bet,
        }
    }

    pub fn river(self) -> PokerGame {
        let mut rng = rand::rng();

        let (deck, new_dealer_card) = self.deck.draw_random_card(&mut rng);
        let dealer_hand = self.dealer_hand.add_card(&new_dealer_card);

        PokerGame {
            players: self.players,
            dealer: self.dealer,
            dealer_hand,
            deck,
            state: PokerState::RiverRound,
            min_bet: self.min_bet,
        }
    }

    pub fn step(self) -> PokerGame {
        match self.state {
            PokerState::PreFlopRound => todo!(),
            PokerState::Flop => self.flop(),
            PokerState::FlopRound => todo!(),
            PokerState::Turn => self.turn(),
            PokerState::TurnRound => todo!(),
            PokerState::River => self.river(),
            PokerState::RiverRound => todo!(),
            PokerState::GameOver(_) => todo!(),
        }
    }
}
