#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceType,
}
impl Piece {
    pub fn new(color: PieceColor, kind: PieceType) -> Self {
        Self { color, kind }
    }

    pub fn to_char(&self) -> char {
        let char = match self.kind {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
        };

        match self.color {
            PieceColor::White => char.to_ascii_uppercase(),
            PieceColor::Black => char.to_ascii_lowercase(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn other(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
