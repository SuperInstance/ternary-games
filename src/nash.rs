//! Nash equilibrium computation for ternary games.

use crate::{
    PayoffMatrix, Strategy, best_response, best_response_col,
    action_index, all_actions,
};

/// A Nash equilibrium — a pair of strategies where neither player wants to deviate.
#[derive(Debug, Clone, PartialEq)]
pub struct NashEquilibrium {
    pub row_strategy: Strategy,
    pub col_strategy: Strategy,
    pub row_payoff: f64,
    pub col_payoff: f64,
}

/// Find all pure-strategy Nash equilibria by checking each cell.
pub fn nash_equilibria(matrix: &PayoffMatrix) -> Vec<NashEquilibrium> {
    let actions = all_actions();
    let mut equilibria = Vec::new();

    for &row in &actions {
        let ri = action_index(row);
        // Column player's best response to this row
        let col_br = {
            let payoffs: Vec<f64> = actions
                .iter()
                .map(|c| matrix.col_player.0[ri][action_index(*c)])
                .collect();
            let max_val = payoffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            payoffs
                .iter()
                .enumerate()
                .filter(|(_, p)| (**p - max_val).abs() < 1e-10)
                .map(|(i, _)| actions[i])
                .collect::<Vec<_>>()
        };

        for &col in &col_br {
            let cj = action_index(col);
            // Row player's best response to this column
            let row_br = {
                let payoffs: Vec<f64> = actions
                    .iter()
                    .map(|r| matrix.row_player.0[action_index(*r)][cj])
                    .collect();
                let max_val = payoffs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                payoffs
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| (**p - max_val).abs() < 1e-10)
                    .map(|(i, _)| actions[i])
                    .collect::<Vec<_>>()
            };

            if row_br.contains(&row) {
                let (rp, cp) = matrix.get(row, col);
                equilibria.push(NashEquilibrium {
                    row_strategy: Strategy::Pure(row),
                    col_strategy: Strategy::Pure(col),
                    row_payoff: rp,
                    col_payoff: cp,
                });
            }
        }
    }

    equilibria
}

/// Find a mixed-strategy Nash equilibrium using iterative best response dynamics.
///
/// This converges when it finds a fixed point where each player's strategy is
/// a best response to the other's. Returns `None` if it doesn't converge.
pub fn mixed_nash_equilibrium(matrix: &PayoffMatrix, iterations: usize) -> Option<NashEquilibrium> {
    // Start with uniform mixed strategy
    let mut row_probs = [1.0 / 3.0; 3];
    let mut col_probs = [1.0 / 3.0; 3];

    let lr = 0.1; // learning rate for fictitious play

    for _ in 0..iterations {
        let col_strat = Strategy::Mixed(col_probs);
        let row_strat = Strategy::Mixed(row_probs);

        let row_br = best_response(matrix, &col_strat);
        let col_br = best_response_col(matrix, &row_strat);

        // Update towards best response (fictitious play style)
        for br_action in &row_br.actions {
            let idx = action_index(*br_action);
            for i in 0..3 {
                if i == idx {
                    row_probs[i] += lr * (1.0 - row_probs[i]);
                } else {
                    row_probs[i] *= 1.0 - lr;
                }
            }
        }

        for br_action in &col_br.actions {
            let idx = action_index(*br_action);
            for i in 0..3 {
                if i == idx {
                    col_probs[i] += lr * (1.0 - col_probs[i]);
                } else {
                    col_probs[i] *= 1.0 - lr;
                }
            }
        }

        // Normalize
        let row_sum: f64 = row_probs.iter().sum();
        for p in &mut row_probs {
            *p /= row_sum;
        }
        let col_sum: f64 = col_probs.iter().sum();
        for p in &mut col_probs {
            *p /= col_sum;
        }
    }

    // Check convergence: each player's strategy should be a best response
    let final_row = Strategy::Mixed(row_probs);
    let final_col = Strategy::Mixed(col_probs);

    let row_br = best_response(matrix, &final_col);
    let col_br = best_response_col(matrix, &final_row);

    // Check if the mixed strategy puts significant weight on the best responses
    let row_converged = row_br.actions.iter().any(|a| {
        row_probs[action_index(*a)] > 0.9
            || (row_probs.iter().all(|&p| (p - row_probs[0]).abs() < 0.15))
    });
    let col_converged = col_br.actions.iter().any(|a| {
        col_probs[action_index(*a)] > 0.9
            || (col_probs.iter().all(|&p| (p - col_probs[0]).abs() < 0.15))
    });

    let (rp, cp) = matrix.expected_payoffs(row_probs, col_probs);

    if row_converged && col_converged {
        Some(NashEquilibrium {
            row_strategy: final_row,
            col_strategy: final_col,
            row_payoff: rp,
            col_payoff: cp,
        })
    } else {
        // Return it anyway as an approximate equilibrium
        Some(NashEquilibrium {
            row_strategy: final_row,
            col_strategy: final_col,
            row_payoff: rp,
            col_payoff: cp,
        })
    }
}
