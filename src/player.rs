use crate::deck::Hand;

#[derive(Hash)]
enum PlayerState {
    Playing(u64, Hand),
    Folded,
}

#[derive(Hash)]
pub struct Player {
    state: PlayerState,
    money: u64,
}

impl Player {
    pub fn from_money(money: u64) -> Self {
        todo!();
    }

    pub fn from_money_and_hand(money: u64, hand: Hand) -> Self {
        todo!();
    }

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

#[cfg(test)]
mod test {
    use crate::player::Player;

    #[test]
    fn from_money_1() {
        let new_player = Player::from_money(1_000);

        assert!(new_player.money == 1_000);
    }
}
