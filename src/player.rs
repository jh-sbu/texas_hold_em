use crate::deck::Hand;

enum PlayerState {
    Playing(u64, Hand),
    Folded,
}

pub struct Player {
    state: PlayerState,
    money: u64,
}

impl Player {
    pub fn call(&self) -> Player {
        todo!();
    }

    pub fn fold(&self) -> Player {
        todo!();
    }

    pub fn raise(&self) -> Player {
        todo!();
    }

    pub fn all_in(&self) -> Player {
        todo!();
    }
}
