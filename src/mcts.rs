use std::{collections::HashMap, f64::consts::SQRT_2, ops::Index};

use crate::{
    player::{Player, PlayerAction},
    poker::PokerGame,
};

const UCT_C: f64 = SQRT_2;

struct StateNode {
    call: ActionNode,
    fold: ActionNode,
    raise: ActionNode,
    all_in: ActionNode,
    visits: f64,
    needs_exploration: bool,
}

fn uct(state_node: &StateNode, action_node: &ActionNode) -> f64 {
    action_node.payout / action_node.cost
        + UCT_C * f64::sqrt(state_node.visits.ln() / action_node.cost)
}

impl Index<&PlayerAction> for StateNode {
    type Output = ActionNode;

    fn index(&self, index: &PlayerAction) -> &Self::Output {
        match index {
            PlayerAction::Fold => &self.fold,
            PlayerAction::Call => &self.call,
            PlayerAction::Raise => &self.raise,
            PlayerAction::AllIn => &self.all_in,
        }
    }
}

impl Index<PlayerAction> for StateNode {
    type Output = ActionNode;

    fn index(&self, index: PlayerAction) -> &Self::Output {
        &self[&index]
    }
}

impl StateNode {
    fn new() -> Self {
        Self {
            call: ActionNode::new(),
            fold: ActionNode::new(),
            raise: ActionNode::new(),
            all_in: ActionNode::new(),
            visits: 0.,
            needs_exploration: true,
        }
    }

    fn get_next_action(&self) -> PlayerAction {
        if self.needs_exploration {
            let possible_actions: Vec<_> = [
                PlayerAction::Fold,
                PlayerAction::Call,
                PlayerAction::Raise,
                PlayerAction::AllIn,
            ]
            .into_iter()
            .filter(|action| self[action].visited)
            .collect();
        }
        todo!();
    }
}

struct ActionNode {
    payout: f64,
    cost: f64,
    visited: bool,
}

impl ActionNode {
    fn new() -> Self {
        Self {
            payout: 0.,
            cost: 0.,
            visited: false,
        }
    }
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
