use std::collections::HashMap;

use crate::poker::PokerGame;

struct StateNode;

struct ActionNode;

struct Mcts {
    nodes: HashMap<PokerGame, StateNode>,
}

impl Mcts {
    pub fn from_players() -> Self {
        todo!();
    }

    pub fn new() -> Self {
        todo!();
    }

    pub fn selection(&self) -> Self {
        todo!();
    }

    pub fn expansion(&self) -> Self {
        todo!();
    }

    pub fn backprop(&self) -> Self {
        todo!();
    }
}
