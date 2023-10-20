use crate::game::{Game, PlayerSide};
use crate::solver::SequenceTree;

pub mod game;
pub mod solver;

fn main() {
    let game = Game::new();
    let mut tree = SequenceTree::new(game.clone());
    tree.generate_tree(PlayerSide::Player, PlayerSide::Player, 0);
    // get user input
    //println!("{} nodes", tree.nodes.len());
}
