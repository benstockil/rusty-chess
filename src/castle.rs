use crate::piece::PieceColor;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CastlingRights {
    pub white: PlayerCastlingRights,
    pub black: PlayerCastlingRights,
}

impl CastlingRights {
    pub fn get(&self, color: PieceColor) -> &PlayerCastlingRights {
        match color {
            PieceColor::White => &self.white,
            PieceColor::Black => &self.black,
        }
    }

    pub fn get_mut(&mut self, color: PieceColor) -> &mut PlayerCastlingRights {
        match color {
            PieceColor::White => &mut self.white,
            PieceColor::Black => &mut self.black,
        }
    }

    pub(crate) fn default() -> CastlingRights {
        Self {
            white: PlayerCastlingRights::default(),
            black: PlayerCastlingRights::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PlayerCastlingRights {
    pub queenside: bool,
    pub kingside: bool,
}

impl PlayerCastlingRights {
    pub fn forbid_queenside(&mut self) {
        self.queenside = false;
    }

    pub fn forbid_kingside(&mut self) {
        self.kingside = false;
    }

    pub fn forbid_all(&mut self) {
        self.queenside = false;
        self.kingside = false;
    }

    fn default() -> PlayerCastlingRights {
        Self {
            queenside: true,
            kingside: true,
        }
    }
}
