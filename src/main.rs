use std::io::Write;

use crate::game::{Game, PlayerSide};
use crate::solver::{EvalMethod, SequenceTree};

pub mod game;
pub mod minimax;
pub mod qlearning_move;
pub mod solver;
pub mod test;

fn main() {
    let mut game = Game::default();
    let mut stash = Game::default();
    let mut tree = SequenceTree::new(game); // Check if game needs to be cloned
    tree.generate_tree(PlayerSide::Player, None);
    // get user input
    //println!("{} nodes", tree.nodes.len());
    loop {
        // Main menu has the following options:
        // - Reset Game
        // - Manually Enter Board State
        // - Stash Game
        // - Load Game
        // - Test move (displays resulting board)
        // - Play move
        // - Generate Sequence Tree
        // - Solve Game (not yet implemented)

        // get user input
        println!("Main Menu");
        println!("(D)isplay Current Game");
        println!("(R)eset Game");
        println!("(M)anually Enter Board State");
        println!("(S)tash Game");
        println!("(L)oad Game");
        println!("(T)est Move");
        println!("(P)lay Move");
        println!("(G)enerate Sequence Tree");
        println!("(F)ind best move");
        let mut input = String::new();
        print!("> ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        match input.as_str() {
            "d" => {
                println!("{:?}", game);
            }
            "r" => {
                game = Game::default();
                println!("{:?}", game);
            }
            "m" => {
                println!("Enter the player side pockets:");
                let mut input = String::new();
                print!("> ");
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();
                let player_pockets: [i32; 7] = input
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                println!("Enter the opponent side pockets:");
                let mut input = String::new();
                print!("> ");
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();
                let opponent_pockets: [i32; 7] = input
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                println!("Select current player turn: \n1: Player\n2: Opponent");
                let mut input = String::new();
                print!("> ");
                std::io::stdin().read_line(&mut input).unwrap();
                // let trimmed_input = input.trim().to_lowercase().as_str();
                let player_turn: PlayerSide = match input.trim().to_lowercase().as_str() {
                    "1" => PlayerSide::Player,
                    "2" => PlayerSide::Opponent,
                    &_ => PlayerSide::Player,
                };
                game = Game::new(game::Board {
                    player_pockets,
                    opponent_pockets,
                    player_turn,
                });
            }
            "s" => {
                stash = game;
                println!("Stashed game");
            }
            "l" => {
                game = stash;
                println!("Loaded game");
            }
            "t" => {
                let mut test_game = game;
                println!("Enter the pocket to test:");
                let mut input = String::new();
                print!("> ");
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();
                match input.parse::<usize>() {
                    Ok(n) => match test_game.play_move((n, test_game.board.player_turn)) {
                        Ok(_) => println!("{:?}", test_game),
                        Err(_) => println!("Invalid input"),
                    },
                    Err(_) => {
                        println!("Invalid input");
                    }
                }
            }
            "p" => {
                println!("Enter the pocket to play:");
                let mut input = String::new();
                print!("> ");
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();
                match input.parse::<usize>() {
                    Ok(n) => match game.play_move((n, game.board.player_turn)) {
                        Ok(_) => println!("{:?}", game),
                        Err(_) => println!("Invalid input"),
                    },
                    Err(_) => {
                        println!("Invalid input");
                    }
                }
            }
            "g" => {
                println!("Generating sequence tree...");
                tree = SequenceTree::new(game);
                tree.generate_tree(PlayerSide::Player, None);
            }
            "f" => {
                let mut test_game = game;
                println!("Finding best move...");
                let best_sequence = tree.get_best_sequence(&EvalMethod::ByDifference, true, true);
                for pocket in best_sequence {
                    print!("{} ", pocket);
                    test_game
                        .play_move((pocket, test_game.board.player_turn))
                        .unwrap();
                }
                println!();
                println!("{:?}", test_game);
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}
