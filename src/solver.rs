use crate::game::{Game, GameState, PlayerSide, PocketIndex};

pub type SequenceTreeIndex = usize;

/*
The Sequence tree is not represented as a tree in memory, but as a vector of nodes.
Instead of referencing nodes with either pointers or nested structs, the nodes are referenced by their index in the vector.
struct SequenceTree { // The tree itself
    nodes: Vec< // The nodes in the tree
        struct SequenceNode { // A node in the tree
            node_enum: enum SequenceNodeEnum { // The enum that determines the type of node
                Root(Game), // The root node
                Move(
                    struct MoveNode { // A node that represents a move
                        r#move: Move, // The move that this node represents
                        parent: usize, // The index of the parent node in the nodes vector
                    },
                ), // A node that represents a move
            },
        },
    >,
    leaf_nodes: Vec<usize>,
    game_over_nodes: Vec<usize>,
}
*/

/// A move that can be made in the game
#[derive(Clone, Copy)]
pub struct Move {
    /// The pocket that the move is made from
    pocket: PocketIndex,
    /// Resulting score of the move
    pub score: i32,
    /// Whether or not the player gets a free turn
    free_turn: bool,
    /// The resulting game state after the move is made
    pub game: Game,
}

/// Tree containing all possible sequences of moves for a given turn
pub struct SequenceTree {
    /// The nodes in the tree
    pub nodes: Vec<SequenceNode>,
    /// The indices of the leaf nodes in the nodes vector; used to find the end of a sequence
    pub leaf_nodes: Vec<SequenceTreeIndex>,
    /// The indices of the game over nodes in the nodes vector; used to find game ending sequences
    pub game_over_nodes: Vec<SequenceTreeIndex>,
}

#[derive(Clone)]
/// A variant of SequenceNodeEnum that represents a move node
pub struct MoveNode {
    /// The move that this node represents
    pub r#move: Move,
    /// The index of the parent node in the nodes vector
    parent: SequenceTreeIndex,
}

#[derive(Clone)]
/// An enum that represents the different types of nodes in the sequence tree
pub enum SequenceNodeEnum {
    /// The root node which contains the initial game state
    Root(Game),
    /// A node that represents a move
    Move(MoveNode),
}

#[derive(Clone)]
/// A node in the sequence tree
pub struct SequenceNode {
    /// The data contained in the node (either a root node or a move node)
    pub node_enum: SequenceNodeEnum,
    /// Indices of the children of the node in the nodes vector
    children: Vec<SequenceTreeIndex>,
    /// Depth of the node in the tree (calculated from the root node)
    depth: usize,
    /// Path to the node from the root node (usize indices of the nodes in the nodes vector)
    pub path: Vec<SequenceTreeIndex>,
}

impl Game {
    fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for pocket in 0..6 {
            if self.board.get_stones((pocket, self.board.player_turn)) == 0 {
                continue;
            }
            let mut game = *self; // May need to be cloned
            game.play_move((pocket, self.board.player_turn))
                .expect("Invalid move");
            moves.push(Move {
                pocket,
                score: game.board.get_stones((6, self.board.player_turn)),
                free_turn: game.board.player_turn == self.board.player_turn,
                game, // May need to be cloned
            });
        }
        moves
    }
}

impl SequenceTree {
    pub fn new(game: Game) -> Self {
        let root = SequenceNode {
            node_enum: SequenceNodeEnum::Root(game),
            children: Vec::new(),
            depth: 0,
            path: Vec::new(),
        };
        SequenceTree {
            nodes: vec![root],
            leaf_nodes: vec![],
            game_over_nodes: vec![],
        }
    }

    /// for each move, create a new node, push it to the nodes vector, and add the index to the parent's children vector
    fn create_children(&mut self, moves: Vec<Move>, parent_index: SequenceTreeIndex) {
        for r#move in moves {
            let node = SequenceNode {
                node_enum: SequenceNodeEnum::Move(MoveNode {
                    r#move,
                    parent: parent_index,
                }),
                children: Vec::new(),
                depth: match self.nodes[parent_index].node_enum {
                    SequenceNodeEnum::Root(_) => 1,
                    SequenceNodeEnum::Move(ref move_node) => self.nodes[move_node.parent].depth + 1,
                },
                path: {
                    let mut path = self.nodes[parent_index].path.clone();
                    path.push(parent_index);
                    path
                },
            };
            self.nodes.push(node);
            let child_index = self.nodes.len() - 1;
            self.nodes[parent_index].children.push(child_index);
            // if the turn is over or the game is over (even by technicality), add the index to the leaf_nodes vector
            if !r#move.free_turn || r#move.game.game_state != GameState::InProgress {
                self.leaf_nodes.push(child_index);
            }
            if r#move.game.game_state != GameState::InProgress {
                self.game_over_nodes.push(child_index);
            }
        }
    }

    /// Recursively generate the sequence tree
    pub fn generate_tree(
        &mut self,
        player_turn: PlayerSide,
        parent_index: Option<SequenceTreeIndex>,
    ) {
        let parent_index = parent_index.unwrap_or(0);
        let game = match self.nodes[parent_index].node_enum {
            SequenceNodeEnum::Root(ref game) => *game, // May need to be cloned
            SequenceNodeEnum::Move(ref move_node) => move_node.r#move.game, // May need to be cloned
        };
        // base case: if the game or turn is over, return
        if game.game_state != GameState::InProgress || game.board.player_turn != player_turn {
            return;
        }
        let moves = game.possible_moves();
        self.create_children(moves, parent_index);
        // recursively generate the tree for each new child=
        self.nodes[parent_index]
            .children
            .clone()
            .iter()
            .for_each(|child_index| {
                let child = &self.nodes[*child_index];
                match child.node_enum {
                    SequenceNodeEnum::Move(_) => {
                        self.generate_tree(player_turn, Some(*child_index));
                    }
                    // This realistically should never happen since each child created will always bs the Move variant
                    _ => panic!("Child node is not a move node"),
                }
            });
    }

    pub fn get_move_sequence(&self, node_index: SequenceTreeIndex) -> Vec<PocketIndex> {
        let mut move_sequence = Vec::new();
        for index in &self.nodes[node_index].path {
            if let SequenceNodeEnum::Move(ref move_node) = self.nodes[*index].node_enum {
                move_sequence.push(move_node.r#move.pocket)
            }
        }
        move_sequence.push(match self.nodes[node_index].node_enum {
            SequenceNodeEnum::Move(ref move_node) => move_node.r#move.pocket,
            _ => panic!("Leaf node is not a move node"),
        });
        move_sequence
    }

    pub fn get_best_sequence(
        &self,
        eval_method: &EvalMethod,
        prefer_win: bool,
        maximize: bool,
    ) -> Vec<PocketIndex> {
        let mut best_sequence = Vec::new();
        let mut best_evaluation = match maximize {
            true => f32::NEG_INFINITY,
            false => f32::INFINITY,
        };
        let comparison = match maximize {
            true => f32::gt,
            false => f32::lt,
        };
        let filtered_leaf_nodes = match prefer_win {
            true => {
                if self.game_over_nodes.is_empty() {
                    &self.leaf_nodes
                } else {
                    &self.game_over_nodes
                }
            }
            false => &self.leaf_nodes,
        };
        for index in filtered_leaf_nodes {
            let game = match self.nodes[*index].node_enum {
                SequenceNodeEnum::Move(ref move_node) => move_node.r#move.game,
                _ => panic!("Leaf node is not a move node"),
            };
            let evaluation = evaluate(&game, eval_method);
            if comparison(&evaluation, &best_evaluation) {
                best_evaluation = evaluation;
                best_sequence = self.get_move_sequence(*index);
            }
        }
        best_sequence
    }
}

pub enum EvalMethod {
    ByDifference,
}

/// Evaluate a game state by the difference in score between the two players
fn eval_by_difference(game: &Game) -> f32 {
    game.board.get_stones((6, PlayerSide::Player)) as f32
        - game.board.get_stones((6, PlayerSide::Opponent)) as f32
}

pub fn evaluate(game: &Game, eval_method: &EvalMethod) -> f32 {
    match eval_method {
        EvalMethod::ByDifference => eval_by_difference(game),
    }
}
