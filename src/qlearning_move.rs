use rurel::mdp::{State, Agent};
use crate::game::{Game, PlayerSide, PocketIndex};

impl State for Game {
    type A = PocketIndex;
    fn actions(&self) -> Vec<Self::A> {
        (0..6)
        .filter(|pocket: &PocketIndex| self.board.get_stones((*pocket, self.board.player_turn)) > 0)
        .collect()
    }

    fn reward(&self) -> f64 {
        todo!()
    }
}

struct MyAgent {
    state: Game,
}

impl Agent<Game> for MyAgent {
    fn current_state(&self) -> &Game {
        &self.state
    }

    fn take_action(&mut self, action: &usize) {
        self.state.play_move((*action, self.state.board.player_turn)).expect("Invalid move");
    }
}
