//! Payoff matrix for two-player ternary games.

use crate::{PlayerPayoff, TernaryAction, all_actions};

/// A two-player normal-form game with ternary actions.
///
/// Each player has a 3×3 payoff matrix. The row player chooses a row,
/// the column player chooses a column, and both receive their respective payoffs.
#[derive(Debug, Clone, PartialEq)]
pub struct PayoffMatrix {
    /// Row player's payoffs.
    pub row_player: PlayerPayoff,
    /// Column player's payoffs.
    pub col_player: PlayerPayoff,
}

impl PayoffMatrix {
    /// Create a new payoff matrix from two 3×3 arrays.
    pub fn new(row_player: [[f64; 3]; 3], col_player: [[f64; 3]; 3]) -> Self {
        Self {
            row_player: PlayerPayoff::new(row_player),
            col_player: PlayerPayoff::new(col_player),
        }
    }

    /// Get payoffs for a given action pair: (row_payoff, col_payoff).
    pub fn get(&self, row: TernaryAction, col: TernaryAction) -> (f64, f64) {
        (
            self.row_player.get(row, col),
            self.col_player.get(row, col),
        )
    }

    /// Get the row player's payoff for a specific cell.
    pub fn row_payoff(&self, row: TernaryAction, col: TernaryAction) -> f64 {
        self.row_player.get(row, col)
    }

    /// Get the column player's payoff for a specific cell.
    pub fn col_payoff(&self, row: TernaryAction, col: TernaryAction) -> f64 {
        self.col_player.get(row, col)
    }

    /// Compute the expected payoffs given mixed strategies.
    pub fn expected_payoffs(
        &self,
        row_probs: [f64; 3],
        col_probs: [f64; 3],
    ) -> (f64, f64) {
        (
            self.row_player.expected(row_probs, col_probs),
            self.col_player.expected(row_probs, col_probs),
        )
    }

    /// Find cells that are Pareto optimal (no other cell is better for both players).
    pub fn pareto_optimal(&self) -> Vec<(TernaryAction, TernaryAction)> {
        let actions = all_actions();
        let cells: Vec<_> = actions
            .iter()
            .flat_map(|r| actions.iter().map(move |c| (*r, *c)))
            .collect();

        let payoffs: Vec<(f64, f64)> = cells.iter().map(|(r, c)| self.get(*r, *c)).collect();

        cells
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                payoffs.iter().enumerate().all(|(j, (pr, pc))| {
                    j == *i || !(*pr > payoffs[*i].0 && *pc > payoffs[*i].1)
                })
            })
            .map(|(_, c)| c)
            .collect()
    }

    /// Create a zero-sum game from a single payoff matrix (row player's payoffs).
    pub fn zero_sum(row_player: [[f64; 3]; 3]) -> Self {
        let col_player = [
            [-row_player[0][0], -row_player[0][1], -row_player[0][2]],
            [-row_player[1][0], -row_player[1][1], -row_player[1][2]],
            [-row_player[2][0], -row_player[2][1], -row_player[2][2]],
        ];
        Self::new(row_player, col_player)
    }

    /// Create a symmetric game where both players have the same payoff matrix.
    pub fn symmetric(payoffs: [[f64; 3]; 3]) -> Self {
        Self::new(payoffs, payoffs)
    }
}
