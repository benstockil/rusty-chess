use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::random;

use crate::{
    castle::CastlingRights,
    piece::{Piece, PieceColor, PieceType},
    position::BoardPosition,
    search::Score,
};

lazy_static! {
    static ref BITSTRINGS: [u64; 768] = {
        let mut bitstrings: [u64; 768] = [0; 768];
        bitstrings.fill_with(random);
        bitstrings
    };
    static ref EN_PASSANTS: [u64; 8] = {
        let mut en_passants: [u64; 8] = [0; 8];
        en_passants.fill_with(random);
        en_passants
    };
    static ref CASTLING_RIGHTS: [u64; 16] = {
        let mut rights: [u64; 16] = [0; 16];
        rights.fill_with(random);
        rights
    };
    static ref BLACK_TO_MOVE: u64 = random();
}

#[derive(Debug, Clone)]
pub struct Transposition {
    pub score: Score,
    pub depth: u32,
}

impl Transposition {
    pub fn new(score: Score, depth: u32) -> Self {
        Self { score, depth }
    }
}

#[derive(Debug, Clone)]
pub struct TranspositionTable {
    table: HashMap<ZobristKey, Transposition>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, zobrist_key: &ZobristKey) -> Option<&Transposition> {
        self.table.get(zobrist_key)
    }

    pub fn set(&mut self, zobrist_key: ZobristKey, transposition: Transposition) {
        self.table.insert(zobrist_key, transposition);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZobristKey(u64);
impl ZobristKey {
    pub(crate) fn new() -> Self {
        Self(0)
    }
}

impl ZobristKey {
    pub fn toggle_piece(&mut self, piece: &Piece, position: &BoardPosition) {
        let piece_index = match piece.kind {
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Bishop => 2,
            PieceType::Knight => 3,
            PieceType::Rook => 4,
            PieceType::Pawn => 5,
        };

        let color_index = match piece.color {
            PieceColor::White => 1,
            PieceColor::Black => 2,
        };

        let square_index = position.index() as usize;

        let index = square_index * 12 + piece_index * 2 + color_index;

        self.0 ^= BITSTRINGS[index];
    }

    pub fn toggle_en_passant(&mut self, file: u8) {
        self.0 ^= EN_PASSANTS[file as usize];
    }

    pub fn toggle_castling_rights(&mut self, rights: &CastlingRights) {
        let rights_index = ((rights.white.queenside as usize) << 3)
            + ((rights.white.kingside as usize) << 2)
            + ((rights.white.queenside as usize) << 1)
            + rights.white.kingside as usize;

        self.0 ^= CASTLING_RIGHTS[rights_index];
    }

    pub fn toggle_player(&mut self) {
        self.0 ^= *BLACK_TO_MOVE;
    }
}
