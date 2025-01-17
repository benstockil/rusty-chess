use std::{
    ops::Neg,
    time::{Duration, Instant},
};

use crate::{
    board::{FastBoard, MoveError},
    movement::Move,
    transposition::{Transposition, TranspositionTable},
};

pub enum EndState {
    Checkmate,
    Stalemate,
    ThreeFoldRepetiiton,
}

pub struct MoveEngine {
    transposition_table: TranspositionTable,
}

impl MoveEngine {
    pub fn new() -> Self {
        Self {
            transposition_table: TranspositionTable::new(),
        }
    }

    pub fn iterative_deepening(&mut self, board: &mut FastBoard, max_time: Duration) -> Move {
        let expiry = Instant::now() + max_time;

        let mut best_move = None;
        let mut depth = 0;
        loop {
            println!("Searching depth {}...", depth);
            let Some(depth_best_move) = self.find_best_move(board, depth, expiry) else {
                return best_move.unwrap();
            };

            best_move = Some(depth_best_move);
            depth += 1;
        }
    }

    pub fn get_end_state(&mut self, board: &mut FastBoard) -> Option<EndState> {
        if board.get_repetitions() == 3 {
            return Some(EndState::ThreeFoldRepetiiton);
        }

        let possible_moves = board.calculate_pseudo_moves();
        let mut able_to_move = false;

        for possible_move in possible_moves {
            let movement = board.make_move(possible_move);
            if let Err(MoveError::IllegalMove) = movement {
                continue;
            }

            // println!("{}", possible_move);

            movement.unwrap();
            able_to_move = true;
            board.unmake_last_move();
        }

        if able_to_move {
            None
        } else if board.is_in_check(board.next_to_move) {
            Some(EndState::Checkmate)
        } else {
            Some(EndState::Stalemate)
        }
    }

    pub fn find_best_move(
        &mut self,
        board: &mut FastBoard,
        depth: u32,
        expiry: Instant,
    ) -> Option<Move> {
        let possible_moves = board.calculate_pseudo_moves();

        let mut best_move = None;
        let mut best_score = Score::lowest();

        for possible_move in possible_moves {
            let movement = board.make_move(possible_move);
            if let Err(MoveError::IllegalMove) = movement {
                continue;
            }

            movement.unwrap();

            if best_move.is_none() {
                best_move = Some(possible_move);
            }

            let Some(score) = self.alpha_beta(
                board,
                depth,
                Score::initial_alpha(),
                Score::initial_beta(),
                expiry,
            ) else {
                board.unmake_last_move();
                return None;
            };
            // println!("{}", possible_move);
            let score = -score;

            if score > best_score {
                best_move = Some(possible_move);
                best_score = score;
            }

            board.unmake_last_move();
        }

        Some(best_move.unwrap())
    }

    pub fn alpha_beta(
        &mut self,
        board: &mut FastBoard,
        depth: u32,
        mut alpha: Score,
        beta: Score,
        expiry: Instant,
    ) -> Option<Score> {
        if Instant::now() > expiry {
            return None;
        }

        if board.get_repetitions() == 3 {
            return Some(Score::exact(0));
        }

        if let Some(transposition) = self.transposition_table.get(&board.zobrist_key) {
            let score = transposition.score;
            if transposition.depth >= depth {
                let should_use = match score.bound() {
                    ScoreBound::Exact => true,
                    ScoreBound::UpperBound => score < alpha,
                    ScoreBound::LowerBound => score >= beta,
                };

                if should_use {
                    return Some(score);
                }
            }
        }

        if depth == 0 {
            // print_board(board);
            // dbg!(board.evaluate());
            return Some(self.quiesce(board, alpha, beta));
        }

        let cut = beta.make_exact();

        let mut best_score = Score::lowest();
        let mut able_to_move = false;

        let possible_moves = board.calculate_pseudo_moves();
        for possible_move in possible_moves {
            let movement = board.make_move(possible_move);

            if let Err(MoveError::IllegalMove) = movement {
                continue;
            }

            movement.unwrap();

            able_to_move = true;

            let Some(score) = self.alpha_beta(board, depth - 1, -beta, -alpha, expiry) else {
                board.unmake_last_move();
                return None;
            };
            let score = -score;
            best_score = best_score.max(score);

            board.unmake_last_move();
            alpha = alpha.max(score.make_upper_bound());

            if score >= cut {
                best_score = score.make_lower_bound();
                break;
            }
        }

        let transposition = Transposition::new(best_score, depth);
        self.transposition_table
            .set(board.zobrist_key, transposition);

        Some(if able_to_move {
            best_score
        } else if board.is_in_check(board.next_to_move) {
            Score::lowest()
        } else {
            Score::exact(0)
        })
    }

    pub fn quiesce(&mut self, board: &mut FastBoard, _alpha: Score, _beta: Score) -> Score {
        // match self.get_end_state(board) {
        //     Some(EndState::Checkmate) => Score::lowest(),
        //     Some(EndState::Stalemate) => Score::exact(0),
        //     Some(EndState::ThreeFoldRepetiiton) => Score::exact(0),
        //     None => Score::exact(board.evaluate()),
        // }
        Score::exact(board.evaluate())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(i32);

impl Neg for Score {
    type Output = Score;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Score {
    pub fn lowest() -> Self {
        Self(i32::MIN + 4)
    }

    pub fn initial_alpha() -> Self {
        Self((i32::MIN & !3) + 5)
    }

    pub fn initial_beta() -> Self {
        Self((i32::MAX & !3) - 5)
    }

    pub fn exact(value: i32) -> Self {
        Self(4 * value)
    }

    pub fn make_exact(self) -> Self {
        Self((self.0 + 1) & !3)
    }

    pub fn make_lower_bound(self) -> Self {
        Self(self.make_exact().0 + 1)
    }

    pub fn make_upper_bound(self) -> Self {
        Self(self.make_exact().0 - 1)
    }

    pub fn bound(&self) -> ScoreBound {
        match self.0 & 3 {
            0 => ScoreBound::Exact,
            1 => ScoreBound::LowerBound,
            3 => ScoreBound::UpperBound,
            _ => unreachable!(),
        }
    }
}

pub enum ScoreBound {
    UpperBound,
    Exact,
    LowerBound,
}
