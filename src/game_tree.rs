//! Game tree for sequential ternary games with minimax.

use crate::{TernaryAction, all_actions};

/// A node in a sequential game tree.
#[derive(Debug, Clone)]
pub enum GameNode {
    /// A terminal node with a payoff value.
    Leaf(f64),
    /// A decision node: player chooses among 3 actions, each leading to a subtree.
    Decision {
        /// Which player is making the decision (0 or 1).
        player: usize,
        /// Subtrees for actions A, B, C.
        children: [Box<GameNode>; 3],
    },
}

impl GameNode {
    /// Create a leaf node.
    pub fn leaf(value: f64) -> Self {
        GameNode::Leaf(value)
    }

    /// Create a decision node for a player.
    pub fn decision(player: usize, children: [GameNode; 3]) -> Self {
        GameNode::Decision {
            player,
            children: children.map(Box::new),
        }
    }

    /// Compute the minimax value of this subtree.
    /// Player 0 maximizes, player 1 minimizes.
    pub fn minimax(&self) -> f64 {
        match self {
            GameNode::Leaf(v) => *v,
            GameNode::Decision { player, children } => {
                let values: [f64; 3] = [
                    children[0].minimax(),
                    children[1].minimax(),
                    children[2].minimax(),
                ];
                if *player == 0 {
                    values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                } else {
                    values.iter().cloned().fold(f64::INFINITY, f64::min)
                }
            }
        }
    }

    /// Find the optimal action at this node using minimax.
    /// Returns `None` for leaf nodes.
    pub fn optimal_action(&self) -> Option<TernaryAction> {
        match self {
            GameNode::Leaf(_) => None,
            GameNode::Decision { player, children } => {
                let values: [f64; 3] = [
                    children[0].minimax(),
                    children[1].minimax(),
                    children[2].minimax(),
                ];
                let actions = all_actions();
                if *player == 0 {
                    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    actions
                        .iter()
                        .zip(values.iter())
                        .find(|(_, v)| (**v - max_val).abs() < 1e-10)
                        .map(|(a, _)| *a)
                } else {
                    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
                    actions
                        .iter()
                        .zip(values.iter())
                        .find(|(_, v)| (**v - min_val).abs() < 1e-10)
                        .map(|(a, _)| *a)
                }
            }
        }
    }

    /// Count total nodes in the tree.
    pub fn node_count(&self) -> usize {
        match self {
            GameNode::Leaf(_) => 1,
            GameNode::Decision { children, .. } => {
                1 + children.iter().map(|c| c.node_count()).sum::<usize>()
            }
        }
    }

    /// Get the depth of the tree.
    pub fn depth(&self) -> usize {
        match self {
            GameNode::Leaf(_) => 0,
            GameNode::Decision { children, .. } => {
                1 + children.iter().map(|c| c.depth()).max().unwrap_or(0)
            }
        }
    }
}

/// A complete game tree for sequential ternary games.
#[derive(Debug, Clone)]
pub struct GameTree {
    root: GameNode,
}

impl GameTree {
    /// Create a new game tree from a root node.
    pub fn new(root: GameNode) -> Self {
        Self { root }
    }

    /// Compute the minimax value of the game.
    pub fn minimax_value(&self) -> f64 {
        self.root.minimax()
    }

    /// Get the optimal first action.
    pub fn optimal_first_action(&self) -> Option<TernaryAction> {
        self.root.optimal_action()
    }

    /// Get the total number of nodes.
    pub fn node_count(&self) -> usize {
        self.root.node_count()
    }

    /// Get the depth of the tree.
    pub fn depth(&self) -> usize {
        self.root.depth()
    }

    /// Build a complete game tree of given depth with leaf values from a function.
    pub fn build(depth: usize, leaf_value: impl Fn(usize) -> f64) -> Self {
        Self {
            root: Self::build_node(0, depth, &leaf_value),
        }
    }

    fn build_node(current_depth: usize, max_depth: usize, leaf_value: &impl Fn(usize) -> f64) -> GameNode {
        if current_depth >= max_depth {
            return GameNode::Leaf(leaf_value(current_depth));
        }
        let player = current_depth % 2;
        let d = current_depth + 1;
        GameNode::decision(
            player,
            [
                Self::build_node(d, max_depth, leaf_value),
                Self::build_node(d, max_depth, leaf_value),
                Self::build_node(d, max_depth, leaf_value),
            ],
        )
    }
}
