//! Classic games reimagined for ternary agents.
//!
//! The Prisoner's Dilemma with three actions:
//! - A = Cooperate (full cooperation)
//! - B = Defect (standard defection)
//! - C = Partial cooperate (half-measure — cooperate a bit, but not fully)
//!
//! Payoff structure:
//! - Mutual cooperation (A,A) → (3,3)
//! - Mutual defection (B,B) → (1,1)
//! - Mutual partial (C,C) → (2,2)
//! - Defect vs cooperate → defector gets 5, cooperator gets 0
//! - Partial vs cooperate → partial gets 4, cooperator gets 1
//! - Defect vs partial → defector gets 4, partial gets 0.5

use crate::{
    DominanceResult, NashEquilibrium, PayoffMatrix, TernaryAction,
    dominant_strategy, nash_equilibria,
};

/// The ternary Prisoner's Dilemma game.
#[derive(Debug, Clone)]
pub struct TernaryPrisonersGame {
    pub matrix: PayoffMatrix,
}

impl TernaryPrisonersGame {
    /// Create the standard ternary Prisoner's Dilemma.
    pub fn new() -> Self {
        // Row player payoffs:
        //          Col:A    Col:B    Col:C
        // Row:A  [  3.0,     0.0,    1.0  ]
        // Row:B  [  5.0,     1.0,    4.0  ]
        // Row:C  [  4.0,    0.5,    2.0  ]
        let row = [
            [3.0, 0.0, 1.0],
            [5.0, 1.0, 4.0],
            [4.0, 0.5, 2.0],
        ];
        // Symmetric column player payoffs (swap perspective)
        //          Col:A    Col:B    Col:C
        // Row:A  [  3.0,     5.0,    4.0  ]
        // Row:B  [  0.0,     1.0,    0.5  ]
        // Row:C  [  1.0,     4.0,    2.0  ]
        let col = [
            [3.0, 5.0, 4.0],
            [0.0, 1.0, 0.5],
            [1.0, 4.0, 2.0],
        ];

        Self {
            matrix: PayoffMatrix::new(row, col),
        }
    }

    /// Create a custom ternary Prisoner's Dilemma with parameters.
    pub fn with_params(
        reward: f64,      // mutual cooperation
        punishment: f64,   // mutual defection
        temptation: f64,   // defect vs cooperate
        sucker: f64,       // cooperate vs defect
        partial: f64,      // mutual partial
    ) -> Self {
        // Ensure the dilemma structure: temptation > reward > partial > punishment > sucker
        let row = [
            [reward, sucker, (reward + sucker) / 2.0],
            [temptation, punishment, (temptation + punishment) / 2.0],
            [(temptation + reward) / 2.0, (sucker + punishment) / 2.0, partial],
        ];
        let col = [
            [reward, temptation, (temptation + reward) / 2.0],
            [sucker, punishment, (sucker + punishment) / 2.0],
            [(reward + sucker) / 2.0, (temptation + punishment) / 2.0, partial],
        ];

        Self {
            matrix: PayoffMatrix::new(row, col),
        }
    }

    /// Find all Nash equilibria of this game.
    pub fn find_equilibria(&self) -> Vec<NashEquilibrium> {
        nash_equilibria(&self.matrix)
    }

    /// Check if the dilemma property holds: defection should be dominant.
    pub fn has_dilemma_property(&self) -> bool {
        matches!(
            dominant_strategy(&self.matrix),
            DominanceResult::StrictlyDominant(TernaryAction::B)
                | DominanceResult::WeaklyDominant(TernaryAction::B)
        )
    }

    /// Get the socially optimal outcome (maximizes total welfare).
    pub fn socially_optimal(&self) -> (TernaryAction, TernaryAction, f64) {
        let actions = [TernaryAction::A, TernaryAction::B, TernaryAction::C];
        let mut best = (TernaryAction::A, TernaryAction::A, f64::NEG_INFINITY);

        for &r in &actions {
            for &c in &actions {
                let (rp, cp) = self.matrix.get(r, c);
                let total = rp + cp;
                if total > best.2 {
                    best = (r, c, total);
                }
            }
        }
        best
    }
}

impl Default for TernaryPrisonersGame {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to create the standard ternary Prisoner's Dilemma.
pub fn ternary_prisoners_dilemma() -> TernaryPrisonersGame {
    TernaryPrisonersGame::new()
}
