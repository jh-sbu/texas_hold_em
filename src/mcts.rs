use std::collections::HashMap;

use crate::{deck::Hand, player::Player, poker::PokerGame};

struct StateNode {
    call: ActionNode,
    fold: ActionNode,
    raise: ActionNode,
    all_in: ActionNode,
}

struct ActionNode {
    value: f64,
    n: f64,
}

pub struct Mcts {
    nodes: HashMap<PokerGame, StateNode>,
    players: Vec<Player>,
    dealer: usize,
}

impl Mcts {
    pub fn from_players_and_dealer(players: Vec<Player>, dealer: usize) -> Self {
        Self {
            nodes: HashMap::new(),
            players,
            dealer,
        }
    }

    fn rollout(&mut self) {
        let game = PokerGame::from_players_and_dealer(self.players.clone(), self.dealer);

        game.step();

        todo!();
    }

    pub fn new() -> Self {
        todo!();
    }

    pub fn selection(&self) {
        todo!();
    }

    pub fn expansion(&self) {
        todo!();
    }

    pub fn backprop(&self) {
        todo!();
    }
}
