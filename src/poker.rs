use crate::player::Player;

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

struct Players {
    players: Vec<Player>,
    current_dealer: usize,
}

struct PokerGame {
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
