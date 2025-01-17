use std::fmt::Display;

use crate::{castle::CastlingRights, piece::PieceType, position::BoardPosition};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Move {
    Direct {
        from: BoardPosition,
        to: BoardPosition,
        promotion: Option<Promotion>,
    },
    Castle(CastleDirection),
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Direct {
                from,
                to,
                promotion,
            } => {
                let promotion = promotion
                    .map(|p| format!("(promoted to {:?})", p))
                    .unwrap_or("".into());

                write!(f, "{} -> {} {}", from, to, promotion)
            }

            Move::Castle(castle_direction) => match castle_direction {
                CastleDirection::QueenSide => write!(f, "castle queenside"),
                CastleDirection::KingSide => write!(f, "castle kingside"),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CastleDirection {
    QueenSide,
    KingSide,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Promotion {
    Queen,
    Knight,
    Bishop,
    Rook,
}

impl Promotion {
    pub fn piece_type(&self) -> PieceType {
        match self {
            Promotion::Queen => PieceType::Queen,
            Promotion::Knight => PieceType::King,
            Promotion::Rook => PieceType::Rook,
            Promotion::Bishop => PieceType::Bishop,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PastMove {
    pub move_made: Move,
    pub captured: Option<PieceType>,
    pub previous_castling_rights: Option<CastlingRights>,
    pub previous_en_passant: Option<u8>,
}

impl PastMove {
    pub fn new(
        move_made: Move,
        captured: Option<PieceType>,
        previous_castling_rights: Option<CastlingRights>,
        previous_en_passant: Option<u8>,
    ) -> Self {
        Self {
            move_made,
            captured,
            previous_castling_rights,
            previous_en_passant,
        }
    }
}
