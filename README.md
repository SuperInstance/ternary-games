# ternary-games

Game theory for ternary agents — payoff matrices, Nash equilibria, and strategic reasoning.

## Why Ternary?

Traditional game theory focuses on binary interactions (cooperate/defect). But many real-world scenarios involve **three meaningful choices**:

- **Cooperate / Defect / Partial** — the extended Prisoner's Dilemma
- **Rock / Paper / Scissors** — three-way cyclic dominance
- **Attack / Defend / Retreat** — military strategy
- **Buy / Sell / Hold** — market decisions

Ternary game theory enriches classical results with a third action, capturing nuanced strategic behavior that binary models miss.

## Features

| Component | Description |
|---|---|
| `PayoffMatrix` | Configurable 3×3 payoff matrix for two-player ternary games |
| `NashEquilibrium` | Find pure and approximate mixed Nash equilibria |
| `BestResponse` | Compute best response to opponent's strategy |
| `DominantStrategy` | Identify strictly/weakly dominant strategies |
| `GameTree` | Minimax game tree for sequential ternary games |
| `PrisonersDilemma` | Classic Prisoner's Dilemma reimagined with 3 actions |

## Quick Start

```rust
use ternary_games::*;

// Create a payoff matrix
let game = PayoffMatrix::new(
    [[3.0, 0.0, 1.0], [5.0, 1.0, 4.0], [4.0, 0.5, 2.0]],
    [[3.0, 5.0, 4.0], [0.0, 1.0, 0.5], [1.0, 4.0, 2.0]],
);

// Find Nash equilibria
let equilibria = nash_equilibria(&game);
for eq in &equilibria {
    println!("Nash eq: row={:?} col={:?}", eq.row_strategy, eq.col_strategy);
}

// Compute best response
let br = best_response(&game, &Strategy::Pure(TernaryAction::A));
println!("Best response to A: {:?} (payoff={})", br.actions, br.payoff);

// Check for dominant strategies
match dominant_strategy(&game) {
    DominanceResult::StrictlyDominant(a) => println!("Dominant: {:?}", a),
    DominanceResult::WeaklyDominant(a) => println!("Weakly dominant: {:?}", a),
    DominanceResult::None => println!("No dominant strategy"),
}
```

## Ternary Prisoner's Dilemma

The classic dilemma extended with a "partial cooperation" option:

| | Coop (A) | Defect (B) | Partial (C) |
|---|---|---|---|
| **Coop (A)** | (3, 3) | (0, 5) | (1, 4) |
| **Defect (B)** | (5, 0) | (1, 1) | (4, 0.5) |
| **Partial (C)** | (4, 1) | (0.5, 4) | (2, 2) |

Defection remains the dominant strategy, but partial cooperation creates a middle ground that changes the strategic landscape.

```rust
use ternary_games::*;

let game = ternary_prisoners_dilemma();
assert!(game.has_dilemma_property()); // Defection is dominant

let (action, _, welfare) = game.socially_optimal();
println!("Socially optimal: {:?} with total welfare {}", action, welfare);
// Output: Socially optimal: A with total welfare 6.0
```

## Sequential Games

Build game trees for turn-based ternary games with minimax:

```rust
use ternary_games::*;

let tree = GameTree::new(GameNode::decision(0, [
    GameNode::decision(1, [
        GameNode::Leaf(3.0), GameNode::Leaf(5.0), GameNode::Leaf(2.0)
    ]),
    GameNode::decision(1, [
        GameNode::Leaf(1.0), GameNode::Leaf(4.0), GameNode::Leaf(6.0)
    ]),
    GameNode::decision(1, [
        GameNode::Leaf(7.0), GameNode::Leaf(3.0), GameNode::Leaf(8.0)
    ]),
]));

println!("Minimax value: {}", tree.minimax_value()); // 3.0
println!("Optimal first move: {:?}", tree.optimal_first_action()); // C
```

## API Overview

- **`PayoffMatrix::new(row, col)`** — create a 3×3 bimatrix game
- **`PayoffMatrix::zero_sum(payoffs)`** — create a zero-sum game
- **`PayoffMatrix::symmetric(payoffs)`** — create a symmetric game
- **`nash_equilibria(matrix)`** — find all pure Nash equilibria
- **`mixed_nash_equilibrium(matrix, iterations)`** — approximate mixed equilibrium via fictitious play
- **`best_response(matrix, strategy)`** — row player's best response
- **`best_response_col(matrix, strategy)`** — column player's best response
- **`dominant_strategy(matrix)`** — check for dominant strategies (row player)
- **`dominant_strategy_col(matrix)`** — check for dominant strategies (column player)
- **`ternary_prisoners_dilemma()`** — instantiate the ternary Prisoner's Dilemma

## Zero Dependencies

Pure Rust, no unsafe code, no external dependencies. The entire implementation is self-contained.

## License

MIT

## See Also
- **ternary-game-theory** — related
- **ternary-arena** — related
- **ternary-scoring** — related
- **ternary-auction** — related
- **ternary-voting** — related

