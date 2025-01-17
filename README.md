# Rusty Chess Bot

A simple chess bot implemented in Rust.

Good enough to beat my brother :)

## Optimisations

- Alpha-beta search
- Iterative deepening
- Bitboard representation
- Transposition table
- Fast move lookup
- Magic lookups
- Integrated bounds and values
- Piece-square table evaluation

## To Implement

- [ ] Quiescence search
- [ ] Move ordering
- [ ] Partial search utilisation
- [ ] Debug interface
- [ ] GUI?

## Usage

To run, simple execute `cargo run --release`.

For each move:

- `<origin> <destination>`: directly move a piece (e.g. `b1 c3`).
- `castle <side>`: perform a castle (e.g. `castle queenside`).
- `unmake`: rollback board state to previous user move (i.e. also undoes computer move).
