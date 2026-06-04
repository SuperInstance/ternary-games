//! Tests for ternary-games crate.

use ternary_games::*;

#[test]
fn test_payoff_matrix_get() {
    let m = PayoffMatrix::new(
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]],
    );
    assert_eq!(m.get(TernaryAction::A, TernaryAction::A), (1.0, 9.0));
    assert_eq!(m.get(TernaryAction::B, TernaryAction::C), (6.0, 4.0));
    assert_eq!(m.get(TernaryAction::C, TernaryAction::C), (9.0, 1.0));
}

#[test]
fn test_payoff_zero_sum() {
    let m = PayoffMatrix::zero_sum([[1.0, -1.0, 0.0], [2.0, 0.0, -2.0], [0.0, 1.0, 1.0]]);
    assert_eq!(m.get(TernaryAction::A, TernaryAction::A), (1.0, -1.0));
    assert_eq!(m.get(TernaryAction::B, TernaryAction::C), (-2.0, 2.0));
}

#[test]
fn test_payoff_symmetric() {
    let m = PayoffMatrix::symmetric([[3.0, 0.0, 1.0], [5.0, 1.0, 4.0], [4.0, 0.5, 2.0]]);
    assert_eq!(m.get(TernaryAction::A, TernaryAction::B), (0.0, 0.0));
}

#[test]
fn test_expected_payoff() {
    let m = PayoffMatrix::new(
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]],
    );
    let (rp, cp) = m.expected_payoffs([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!((rp - 2.0).abs() < 1e-10);
    assert!((cp - 8.0).abs() < 1e-10);
}

#[test]
fn test_best_response_pure() {
    let m = PayoffMatrix::new(
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]],
    );
    let br = best_response(&m, &Strategy::Pure(TernaryAction::A));
    assert!(br.actions.contains(&TernaryAction::C));
    assert!((br.payoff - 7.0).abs() < 1e-10);
}

#[test]
fn test_best_response_mixed() {
    let m = PayoffMatrix::new(
        [[3.0, 0.0, 5.0], [1.0, 4.0, 2.0], [2.0, 3.0, 1.0]],
        [[3.0, 1.0, 2.0], [0.0, 4.0, 3.0], [5.0, 2.0, 1.0]],
    );
    let br = best_response(&m, &Strategy::Mixed([0.33, 0.33, 0.34]));
    assert!(!br.actions.is_empty());
}

#[test]
fn test_best_response_col() {
    let m = PayoffMatrix::new(
        [[3.0, 0.0, 5.0], [1.0, 4.0, 2.0], [2.0, 3.0, 1.0]],
        [[3.0, 1.0, 2.0], [0.0, 4.0, 3.0], [5.0, 2.0, 1.0]],
    );
    // Col player payoffs when row plays A: col(A)=3, col(B)=1, col(C)=2 → best is A
    let br = best_response_col(&m, &Strategy::Pure(TernaryAction::A));
    assert!(br.actions.contains(&TernaryAction::A));
}

#[test]
fn test_dominant_strategy_strict() {
    // Row B dominates for row player
    let m = PayoffMatrix::new(
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [2.0, 3.0, 4.0]],
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [2.0, 3.0, 4.0]],
    );
    match dominant_strategy(&m) {
        DominanceResult::StrictlyDominant(a) => assert_eq!(a, TernaryAction::B),
        _ => panic!("Expected strictly dominant B"),
    }
}

#[test]
fn test_dominant_strategy_none() {
    // Rock-paper-scissors like — no dominant strategy
    let m = PayoffMatrix::zero_sum([[0.0, -1.0, 1.0], [1.0, 0.0, -1.0], [-1.0, 1.0, 0.0]]);
    assert!(matches!(dominant_strategy(&m), DominanceResult::None));
}

#[test]
fn test_nash_pure_single() {
    // Coordination game: both want to match
    let m = PayoffMatrix::symmetric([[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]]);
    let eq = nash_equilibria(&m);
    assert_eq!(eq.len(), 3); // (A,A), (B,B), (C,C)
    for e in &eq {
        assert!((e.row_payoff - 3.0).abs() < 1e-10);
    }
}

#[test]
fn test_nash_pure_dominant() {
    // B is strictly dominant for both
    let m = PayoffMatrix::symmetric([[1.0, 0.0, 0.0], [2.0, 5.0, 3.0], [0.0, 1.0, 0.0]]);
    let eq = nash_equilibria(&m);
    assert_eq!(eq.len(), 1);
    assert_eq!(eq[0].row_strategy, Strategy::Pure(TernaryAction::B));
    assert_eq!(eq[0].col_strategy, Strategy::Pure(TernaryAction::B));
}

#[test]
fn test_nash_prisoners_dilemma() {
    let game = ternary_prisoners_dilemma();
    let eq = game.find_equilibria();
    assert!(!eq.is_empty());
    // Defection (B) should be a Nash equilibrium
    assert!(eq.iter().any(|e| {
        matches!(
            e.row_strategy,
            Strategy::Pure(TernaryAction::B) | Strategy::Mixed(_)
        )
    }));
}

#[test]
fn test_mixed_nash_returns_result() {
    let m = PayoffMatrix::zero_sum([[0.0, -1.0, 1.0], [1.0, 0.0, -1.0], [-1.0, 1.0, 0.0]]);
    let eq = mixed_nash_equilibrium(&m, 1000);
    assert!(eq.is_some());
}

#[test]
fn test_game_tree_minimax_simple() {
    // P0 maximizes, P1 minimizes
    // P0 chooses between subtrees; best leaf for P0
    let tree = GameTree::new(GameNode::decision(
        0,
        [
            GameNode::Leaf(3.0),
            GameNode::Leaf(5.0),
            GameNode::Leaf(1.0),
        ],
    ));
    assert!((tree.minimax_value() - 5.0).abs() < 1e-10);
    assert_eq!(tree.optimal_first_action(), Some(TernaryAction::B));
}

#[test]
fn test_game_tree_two_level() {
    // P0 (max) → P1 (min) → leaves
    let tree = GameTree::new(GameNode::decision(
        0,
        [
            GameNode::decision(
                1,
                [GameNode::Leaf(3.0), GameNode::Leaf(5.0), GameNode::Leaf(2.0)],
            ),
            GameNode::decision(
                1,
                [GameNode::Leaf(1.0), GameNode::Leaf(4.0), GameNode::Leaf(6.0)],
            ),
            GameNode::decision(
                1,
                [GameNode::Leaf(7.0), GameNode::Leaf(3.0), GameNode::Leaf(8.0)],
            ),
        ],
    ));
    // P1 minimizes: min(3,5,2)=2, min(1,4,6)=1, min(7,3,8)=3
    // P0 maximizes: max(2,1,3)=3
    assert!((tree.minimax_value() - 3.0).abs() < 1e-10);
    assert_eq!(tree.optimal_first_action(), Some(TernaryAction::C));
}

#[test]
fn test_game_tree_build() {
    let tree = GameTree::build(3, |depth| (depth as f64) * 2.0);
    assert_eq!(tree.depth(), 3);
    // At depth 3 with 3 actions per node: 1 + 3 + 9 + 27 = 40 nodes
    assert_eq!(tree.node_count(), 40);
}

#[test]
fn test_prisoners_dilemma_structure() {
    let game = ternary_prisoners_dilemma();
    // Mutual cooperation should give (3,3)
    let (rp, cp) = game.matrix.get(TernaryAction::A, TernaryAction::A);
    assert!((rp - 3.0).abs() < 1e-10);
    assert!((cp - 3.0).abs() < 1e-10);
    // Mutual defection should give (1,1)
    let (rp, cp) = game.matrix.get(TernaryAction::B, TernaryAction::B);
    assert!((rp - 1.0).abs() < 1e-10);
    assert!((cp - 1.0).abs() < 1e-10);
}

#[test]
fn test_prisoners_dilemma_has_dilemma() {
    let game = ternary_prisoners_dilemma();
    assert!(game.has_dilemma_property());
}

#[test]
fn test_prisoners_socially_optimal() {
    let game = ternary_prisoners_dilemma();
    let (r, c, total) = game.socially_optimal();
    // Mutual cooperation should be socially optimal: total = 6
    assert_eq!(r, TernaryAction::A);
    assert_eq!(c, TernaryAction::A);
    assert!((total - 6.0).abs() < 1e-10);
}

#[test]
fn test_pareto_optimal() {
    let m = PayoffMatrix::symmetric([[3.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 3.0]]);
    let pareto = m.pareto_optimal();
    // All three (A,A), (B,B), (C,C) are Pareto optimal
    assert_eq!(pareto.len(), 3);
}

#[test]
fn test_strategy_probabilities() {
    let s = Strategy::Pure(TernaryAction::B);
    assert_eq!(s.probabilities(), [0.0, 1.0, 0.0]);
    let s = Strategy::Mixed([0.2, 0.3, 0.5]);
    assert_eq!(s.probabilities(), [0.2, 0.3, 0.5]);
}

#[test]
fn test_custom_prisoners_dilemma() {
    let game = TernaryPrisonersGame::with_params(3.0, 1.0, 5.0, 0.0, 2.0);
    assert!(game.has_dilemma_property());
}
