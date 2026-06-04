//! Dominant strategy identification for ternary games.

use crate::{PayoffMatrix, TernaryAction, action_index, all_actions};

/// Result of dominant strategy analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum DominanceResult {
    /// A strictly dominant strategy exists (always better than all others).
    StrictlyDominant(TernaryAction),
    /// A weakly dominant strategy exists (at least as good as all others, strictly better than some).
    WeaklyDominant(TernaryAction),
    /// No dominant strategy exists.
    None,
}

/// Check for dominant strategies for the row player.
pub fn dominant_strategy(matrix: &PayoffMatrix) -> DominanceResult {
    let actions = all_actions();

    // Check for strictly dominant
    for &candidate in &actions {
        let ci = action_index(candidate);
        let mut strictly_dominates_all = true;

        for &other in &actions {
            if other == candidate {
                continue;
            }
            let oi = action_index(other);

            // candidate must be strictly better for ALL column choices
            let mut strictly_better_for_all = true;
            for j in 0..3 {
                if matrix.row_player.0[ci][j] <= matrix.row_player.0[oi][j] {
                    strictly_better_for_all = false;
                    break;
                }
            }
            if !strictly_better_for_all {
                strictly_dominates_all = false;
                break;
            }
        }

        if strictly_dominates_all {
            return DominanceResult::StrictlyDominant(candidate);
        }
    }

    // Check for weakly dominant
    for &candidate in &actions {
        let ci = action_index(candidate);
        let mut weakly_dominates_all = true;
        let mut strictly_better_somewhere = false;

        for &other in &actions {
            if other == candidate {
                continue;
            }
            let oi = action_index(other);

            let mut at_least_as_good = true;
            for j in 0..3 {
                if matrix.row_player.0[ci][j] < matrix.row_player.0[oi][j] {
                    at_least_as_good = false;
                    break;
                }
                if matrix.row_player.0[ci][j] > matrix.row_player.0[oi][j] {
                    strictly_better_somewhere = true;
                }
            }

            if !at_least_as_good {
                weakly_dominates_all = false;
                break;
            }
        }

        if weakly_dominates_all && strictly_better_somewhere {
            return DominanceResult::WeaklyDominant(candidate);
        }
    }

    DominanceResult::None
}

/// Check for dominant strategies for the column player.
pub fn dominant_strategy_col(matrix: &PayoffMatrix) -> DominanceResult {
    let actions = all_actions();

    for &candidate in &actions {
        let cj = action_index(candidate);
        let mut strictly_dominates_all = true;

        for &other in &actions {
            if other == candidate {
                continue;
            }
            let oj = action_index(other);

            let mut strictly_better_for_all = true;
            for i in 0..3 {
                if matrix.col_player.0[i][cj] <= matrix.col_player.0[i][oj] {
                    strictly_better_for_all = false;
                    break;
                }
            }
            if !strictly_better_for_all {
                strictly_dominates_all = false;
                break;
            }
        }

        if strictly_dominates_all {
            return DominanceResult::StrictlyDominant(candidate);
        }
    }

    for &candidate in &actions {
        let cj = action_index(candidate);
        let mut weakly_dominates_all = true;
        let mut strictly_better_somewhere = false;

        for &other in &actions {
            if other == candidate {
                continue;
            }
            let oj = action_index(other);

            let mut at_least_as_good = true;
            for i in 0..3 {
                if matrix.col_player.0[i][cj] < matrix.col_player.0[i][oj] {
                    at_least_as_good = false;
                    break;
                }
                if matrix.col_player.0[i][cj] > matrix.col_player.0[i][oj] {
                    strictly_better_somewhere = true;
                }
            }

            if !at_least_as_good {
                weakly_dominates_all = false;
                break;
            }
        }

        if weakly_dominates_all && strictly_better_somewhere {
            return DominanceResult::WeaklyDominant(candidate);
        }
    }

    DominanceResult::None
}
