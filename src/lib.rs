//! # Ternary Games
//!
//! Game theory for ternary agents — payoff matrices, Nash equilibria, and strategic reasoning.
//!
//! Traditional game theory assumes binary choices (cooperate/defect). This crate
//! extends those ideas to **ternary agents** with three possible actions, enabling
//! richer strategic interactions.

mod best_response;
mod dominant;
mod game_tree;
mod nash;
mod payoff;
mod prisoners;

pub use best_response::{best_response, best_response_col, BestResponse};
pub use dominant::{dominant_strategy, dominant_strategy_col, DominanceResult};
pub use game_tree::{GameNode, GameTree};
pub use nash::{mixed_nash_equilibrium, nash_equilibria, NashEquilibrium};
pub use payoff::PayoffMatrix;
pub use prisoners::{ternary_prisoners_dilemma, TernaryPrisonersGame};

/// A ternary action: one of three choices available to an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryAction {
    A,
    B,
    C,
}

/// A player's strategy — either a pure action or a mixed strategy over three actions.
#[derive(Debug, Clone, PartialEq)]
pub enum Strategy {
    /// Play a single action deterministically.
    Pure(TernaryAction),
    /// Play actions with given probabilities [p_a, p_b, p_c] summing to 1.0.
    Mixed([f64; 3]),
}

impl Strategy {
    /// Return the probability vector [p_a, p_b, p_c].
    pub fn probabilities(&self) -> [f64; 3] {
        match self {
            Strategy::Pure(TernaryAction::A) => [1.0, 0.0, 0.0],
            Strategy::Pure(TernaryAction::B) => [0.0, 1.0, 0.0],
            Strategy::Pure(TernaryAction::C) => [0.0, 0.0, 1.0],
            Strategy::Mixed(p) => *p,
        }
    }
}

/// A payoff for a player: (action_row, action_col) → payoff.
/// Stored as a 3×3 array where index 0=A, 1=B, 2=C.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerPayoff(pub [[f64; 3]; 3]);

impl PlayerPayoff {
    pub fn new(payoffs: [[f64; 3]; 3]) -> Self {
        Self(payoffs)
    }

    pub fn get(&self, row: TernaryAction, col: TernaryAction) -> f64 {
        self.0[action_index(row)][action_index(col)]
    }

    /// Compute expected payoff given row and column mixed strategies.
    pub fn expected(&self, row_probs: [f64; 3], col_probs: [f64; 3]) -> f64 {
        let mut total = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                total += row_probs[i] * col_probs[j] * self.0[i][j];
            }
        }
        total
    }
}

/// Convert a TernaryAction to a usize index.
pub fn action_index(a: TernaryAction) -> usize {
    match a {
        TernaryAction::A => 0,
        TernaryAction::B => 1,
        TernaryAction::C => 2,
    }
}

/// All three actions as a slice.
pub fn all_actions() -> [TernaryAction; 3] {
    [TernaryAction::A, TernaryAction::B, TernaryAction::C]
}
