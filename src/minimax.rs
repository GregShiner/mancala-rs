use crate::{solver::{SequenceNodeEnum, SequenceTreeIndex, SequenceTree, evaluate, EvalMethod}, game::{Game, PocketIndex, PlayerSide}};

type GameTreeIndex = usize;
struct GameTreeNode {
    /// The enum that determines the type of node
    node_enum: NodeEnum,
    /// The indices of the children nodes in the nodes vector
    children: Vec<GameTreeIndex>,
    /// The depth of the node in the tree
    depth: usize,
    /// The evaluation of the node (used to determine the minimax value)
    evaluation: f32,
    /// The minimax value of the node
    minimax_value: Option<f32>,
}

struct InteriorNode {
    result: Game,
    parent: GameTreeIndex,
    sequence: Vec<PocketIndex>,
}

enum NodeEnum {
    Root(Game),
    Move(InteriorNode)
}

struct GameTree {
    nodes: Vec<GameTreeNode>,
}

impl GameTreeNode {
    fn from_sequence_node(sequence_index: SequenceTreeIndex, sequence_tree: &SequenceTree, parent_index: GameTreeIndex, game_tree: &GameTree) -> Self {
        let sequence_node = &sequence_tree.nodes[sequence_index];
        let game = match sequence_node.node_enum {
            SequenceNodeEnum::Move(ref move_node) => move_node.r#move.game.clone(),
            _ => panic!("Sequence node is not a move node")
        };
        let interior_node = InteriorNode {
            result: game,
            parent: parent_index,
            sequence: sequence_node.path.clone(),
        };
        let children = Vec::new();
        let evaluation = evaluate(&interior_node.result, &EvalMethod::ByDifference);
        GameTreeNode {
            node_enum: NodeEnum::Move(interior_node),
            children,
            depth: game_tree.nodes[parent_index].depth + 1,
            evaluation,
            minimax_value: None,
        }
    }
}

impl GameTree {
    fn new(mut self, sequence_tree: &SequenceTree) {
        // Create the root node
        let game = match sequence_tree.nodes[0].node_enum {
            SequenceNodeEnum::Root(game) => game,
            _ => panic!("First node is not a root node")
        };
        let root_node = GameTreeNode {
            node_enum: NodeEnum::Root(game),
            children: Vec::new(),
            depth: 0,
            evaluation: evaluate(&game, &EvalMethod::ByDifference),
            minimax_value: None,
        };
        self.nodes.push(root_node);
        // Create the first layer of nodes
        for leaf_index in &sequence_tree.leaf_nodes {
            let game_tree_node = GameTreeNode::from_sequence_node(*leaf_index, sequence_tree, 0, &self);
            self.nodes.push(game_tree_node);
            let index = self.nodes.len() - 1;
            self.nodes[0].children.push(index);
        }
    }

    /// Creates 1 layer of children of the node at the given index
    fn create_children(mut self, index: GameTreeIndex, player_side: PlayerSide) {
        let node = &self.nodes[index];
        let mut sequence_tree = SequenceTree::new(match node.node_enum {
            NodeEnum::Root(ref game) => game.clone(),
            NodeEnum::Move(ref interior_node) => interior_node.result.clone(),
        });
        sequence_tree.generate_tree(player_side, None);
        for leaf_index in &sequence_tree.leaf_nodes {
            let game_tree_node = GameTreeNode::from_sequence_node(*leaf_index, &sequence_tree, index, &self);
            self.nodes.push(game_tree_node);
            let index = self.nodes.len() - 1;
            self.nodes[index].children.push(index);
        }
        
    }
}