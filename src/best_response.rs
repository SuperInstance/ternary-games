//! Best response computation for ternary games.

use crate::{PayoffMatrix, Strategy, TernaryAction, action_index, all_actions};

/// Result of a best response computation.
#[derive(Debug, Clone, PartialEq)]
pub struct BestResponse {
    /// The best response action(s). Multiple if indifferent.
    pub actions: Vec<TernaryAction>,
    /// Expected payoff of the best response.
    pub payoff: f64,
}

/// Compute the row player's best response to the column player's strategy.
pub fn best_response(matrix: &PayoffMatrix, col_strategy: &Strategy) -> BestResponse {
    let col_probs = col_strategy.probabilities();
    let actions = all_actions();

    // Compute expected payoff for each row action
    let payoffs: Vec<f64> = actions
        .iter()
        .map(|a| {
            let mut total = 0.0;
            let ri = action_index(*a);
            for j in 0..3 {
                total += col_probs[j] * matrix.row_player.0[ri][j];
            }
            total
        })
        .collect();

    let max_payoff = payoffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let best_actions: Vec<TernaryAction> = actions
        .iter()
        .zip(payoffs.iter())
        .filter(|(_, p)| (**p - max_payoff).abs() < 1e-10)
        .map(|(a, _)| *a)
        .collect();

    BestResponse {
        actions: best_actions,
        payoff: max_payoff,
    }
}

/// Compute the column player's best response to the row player's strategy.
pub fn best_response_col(matrix: &PayoffMatrix, row_strategy: &Strategy) -> BestResponse {
    let row_probs = row_strategy.probabilities();
    let actions = all_actions();

    let payoffs: Vec<f64> = actions
        .iter()
        .map(|a| {
            let mut total = 0.0;
            let ci = action_index(*a);
            for i in 0..3 {
                total += row_probs[i] * matrix.col_player.0[i][ci];
            }
            total
        })
        .collect();

    let max_payoff = payoffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let best_actions: Vec<TernaryAction> = actions
        .iter()
        .zip(payoffs.iter())
        .filter(|(_, p)| (**p - max_payoff).abs() < 1e-10)
        .map(|(a, _)| *a)
        .collect();

    BestResponse {
        actions: best_actions,
        payoff: max_payoff,
    }
}
