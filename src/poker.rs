use crate::player::Player;

#[derive(Hash)]
enum PokerState {
    Init,
    FirstRound,
    Flop,
    SecondRound,
    Turn,
    ThirdRound,
    River,
    FinalRound,
}

#[derive(Hash)]
struct Players {
    players: Vec<Player>,
    current_dealer: usize,
}

#[derive(Hash)]
pub struct PokerGame {
    players: Vec<Player>,
    state: PokerState,
    min_bet: u64,
}

impl PokerGame {
    pub fn flop(&self) -> PokerGame {
        todo!();
    }

    pub fn turn(&self) -> PokerGame {
        todo!();
    }

    pub fn river(&self) -> PokerGame {
        todo!();
    }
}
