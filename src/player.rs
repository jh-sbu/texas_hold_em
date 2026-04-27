use crate::{deck::Hand, hand_types::HandType};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum PlayerState {
    Playing(Hand),
    Folded,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Player {
    pub(crate) state: PlayerState,
    money: u64,
}

#[derive(Clone, Copy)]
pub(crate) enum PlayerAction {
    Fold,
    Call,
    Raise,
    AllIn,
}

impl Player {
    // Straddles are out of scope so ignore anything
    // before hands are dealt
    pub(crate) fn from_money_and_hand(money: u64, hand: &Hand) -> Self {
        Self {
            state: PlayerState::Playing(*hand),
            money,
        }
    }

    pub(crate) fn player_hand_type(&self) -> HandType {
        todo!();
    }

    pub(crate) fn call(&self) -> Player {
        todo!();
    }

    pub(crate) fn fold(&self) -> Player {
        todo!();
    }

    pub(crate) fn raise(&self) -> Player {
        todo!();
    }

    pub(crate) fn all_in(&self) -> Player {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use crate::{deck::Hand, player::Player};

    // #[test]
    // fn from_money_1() {
    //     let new_player = Player::from_money(1_000);
    //
    //     assert!(new_player.money == 1_000);
    // }
    #[test]
    fn from_money_and_hand_1() {
        let new_player = Player::from_money_and_hand(1_000, &Hand::new_empty());

        assert_eq!(new_player.money, 1_000);
    }
}
