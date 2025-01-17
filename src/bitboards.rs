use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::{
    piece::{Piece, PieceColor, PieceType},
    position::BoardPosition,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn ones() -> Self {
        Self(u64::MAX)
    }

    pub fn from_value(value: u64) -> Self {
        Self(value)
    }

    pub fn from_pos(position: &BoardPosition) -> Self {
        Self(1 << position.index())
    }

    pub fn from_index(index: u8) -> Self {
        Self(1 << index)
    }

    pub fn get(&self, index: u8) -> bool {
        self.0 >> index & 1 == 1
    }

    pub fn set(&mut self, index: u8, value: bool) {
        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }

    pub fn inner(&self) -> u64 {
        self.0
    }

    pub fn shift(self, ranks: i8, files: i8) -> Self {
        let shift = ranks * 8 + files;
        if shift >= 0 {
            Self(self.0 << shift)
        } else {
            Self(self.0 >> -shift)
        }
    }

    pub fn filled_indices(&self) -> impl Iterator<Item = u8> + use<'_> {
        (0..64).filter(|i| self.get(*i))
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn any(&self) -> bool {
        self.0 > 0
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bitboards {
    white: ColorBitboards,
    black: ColorBitboards,
}

impl Bitboards {
    pub fn empty() -> Self {
        Self {
            white: ColorBitboards::empty(),
            black: ColorBitboards::empty(),
        }
    }

    pub fn insert(&mut self, index: u8, piece: Piece) {
        match piece.color {
            PieceColor::White => self.white.set(index, piece.kind, true),
            PieceColor::Black => self.black.set(index, piece.kind, true),
        }
    }

    pub fn remove(&mut self, index: u8, piece: Piece) {
        match piece.color {
            PieceColor::White => self.white.set(index, piece.kind, false),
            PieceColor::Black => self.black.set(index, piece.kind, false),
        }
    }

    pub fn all_pieces(&self) -> Bitboard {
        self.white.any() | self.black.any()
    }

    pub fn get_color(&self, color: PieceColor) -> &ColorBitboards {
        match color {
            PieceColor::White => &self.white,
            PieceColor::Black => &self.black,
        }
    }

    pub fn queens(&self) -> Bitboard {
        self.white.queen & self.black.queen
    }

    pub fn kings(&self) -> Bitboard {
        self.white.king & self.black.king
    }

    pub fn rooks(&self) -> Bitboard {
        self.white.rook & self.black.rook
    }

    pub fn knights(&self) -> Bitboard {
        self.white.knight & self.black.knight
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ColorBitboards {
    pub king: Bitboard,
    pub queen: Bitboard,
    pub bishop: Bitboard,
    pub knight: Bitboard,
    pub rook: Bitboard,
    pub pawn: Bitboard,
}

impl ColorBitboards {
    fn empty() -> ColorBitboards {
        Self {
            king: Bitboard::empty(),
            queen: Bitboard::empty(),
            bishop: Bitboard::empty(),
            knight: Bitboard::empty(),
            rook: Bitboard::empty(),
            pawn: Bitboard::empty(),
        }
    }

    pub fn set(&mut self, index: u8, piece: PieceType, value: bool) {
        match piece {
            PieceType::King => self.king.set(index, value),
            PieceType::Queen => self.queen.set(index, value),
            PieceType::Bishop => self.bishop.set(index, value),
            PieceType::Knight => self.knight.set(index, value),
            PieceType::Rook => self.rook.set(index, value),
            PieceType::Pawn => self.pawn.set(index, value),
        };
    }

    pub fn any(&self) -> Bitboard {
        self.king | self.queen | self.bishop | self.knight | self.rook | self.pawn
    }
}

pub fn print_bitboard(bitboard: Bitboard) {
    println!("Bitboard {:08x}:\n", bitboard.inner());

    for i in 0..8 {
        print!(" {}  ", 8 - i);
        for j in 0..8 {
            let square_char = match bitboard.get(BoardPosition::from_rank_file(7 - i, j).index()) {
                true => 'X',
                false => '.',
            };
            print!("{} ", square_char);
        }
        println!();
    }
    println!("\n    a b c d e f g h\n");
}

// fn translate_bitboard(bitboard: u64) -> u
