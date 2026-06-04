# Future Integration: ternary-games

## Current State
Provides Nash equilibrium computation for two-player ternary games: `PayoffMatrix` with 3×3 payoff matrices per player, `Strategy` enum for ternary actions, `nash_equilibria()` finds all pure-strategy Nash equilibria, and `best_response()` computes optimal counter-strategies.

## Integration Opportunities

### With ternary-consensus (Game-Theoretic Consensus)
The cross-pollination report identifies this directly: Nash equilibrium finding over ternary strategy spaces applies to Byzantine consensus. Define consensus voting as a normal-form game where each voter's payoff depends on outcome and alignment with true preference. The Nash equilibrium IS the consensus — no voter wants to deviate. Byzantine voters are players with adversarial payoff matrices. This could reduce the 3f+1 requirement by treating consensus as a game rather than a protocol.

### With ternary-cell (Strategic Cell Interaction)
Cells in a grid interact through their neighborhoods — each cell's state affects its neighbors. ternary-games models this as a spatial game: each cell plays a ternary game against its neighbors, choosing its state (Neg/Zero/Pos) to maximize fitness given neighbor states. Nash equilibrium over the grid = stable configuration where no cell wants to change. The tick cycle's vibe phase becomes a game round.

### With ternary-thermodynamics (Thermodynamic Games)
ternary-thermodynamics models strategy distributions via Boltzmann statistics. ternary-games provides the payoff structure. Together: the game's Nash equilibrium is the thermodynamic ground state, and the payoff matrix is the Hamiltonian. Temperature controls exploration vs. exploitation in equilibrium finding — high temperature = mixed strategies (exploratory), low temperature = pure strategies (exploitative).

## Potential in Mature Systems
In room-as-codespace, rooms compete for shared resources (compute, LLM proxy calls, user attention). ternary-games models resource allocation as a game: each room is a player, strategies are resource request levels (Neg = release, Zero = maintain, Pos = request more), and payoffs depend on overall system performance. Nash equilibrium resource allocation ensures no room wants to deviate — stable, fair distribution without a central allocator.

## Cross-Pollination Ideas
- **ternary-adversarial**: Game-theoretic adversarial training — the attacker and defender play a zero-sum ternary game. Nash equilibrium strategies are optimal for both.
- **ternary-scheduling**: Schedule games — rooms bid for time slots using ternary strategies. The equilibrium schedule is Pareto-optimal.
- **ternary-federated**: Federated game theory — multiple federated learning nodes play a cooperation game where the Nash equilibrium is the best aggregation strategy.

## Dependencies for Next Steps
- Define `ConsensusGame` mapping voter payoffs to ternary payoff matrices
- Implement spatial game over cell grids in ternary-cell
- Add mixed-strategy Nash equilibrium computation (currently only pure-strategy)
- Build game-theoretic resource allocator for PLATO room scheduling
