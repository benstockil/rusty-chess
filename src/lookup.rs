use crate::{
    bitboards::Bitboard,
    magics::{BISHOP_MAGICS, ROOK_MAGICS},
    masks::{
        bishop_move_mask, bishop_occupancy_mask, generate_obstruction_maps, king_move_mask,
        knight_move_mask, pawn_attack_mask, pawn_move_mask, rook_move_mask, rook_occupancy_mask,
    },
    piece::PieceColor,
    position::BoardPosition,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOOKUP: LookupTables = LookupTables::new();
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTables {
    pawn_lookup: PawnLookup,
    king_lookup: JumpingPieceLookup,
    knight_lookup: JumpingPieceLookup,
    rook_lookup: SlidingPieceLookup<180224>,
    bishop_lookup: SlidingPieceLookup<7296>,
}

impl LookupTables {
    pub fn new() -> Self {
        Self {
            pawn_lookup: PawnLookup::generate(),
            king_lookup: JumpingPieceLookup::generate_king(),
            knight_lookup: JumpingPieceLookup::generate_knight(),
            rook_lookup: SlidingPieceLookup::generate_rook(),
            bishop_lookup: SlidingPieceLookup::generate_bishop(),
        }
    }

    pub fn pawn_lookup(
        &self,
        square: &BoardPosition,
        color: PieceColor,
        piece_mask: Bitboard,
        enemy_mask: Bitboard,
    ) -> Bitboard {
        self.pawn_lookup
            .lookup(square, color, piece_mask, enemy_mask)
    }

    pub fn pawn_attacks(&self, square: &BoardPosition, color: PieceColor) -> Bitboard {
        self.pawn_lookup.attacks(square, color)
    }

    pub fn king_lookup(&self, square: &BoardPosition) -> Bitboard {
        self.king_lookup.lookup(square)
    }

    pub fn knight_lookup(&self, square: &BoardPosition) -> Bitboard {
        self.knight_lookup.lookup(square)
    }

    pub fn rook_lookup(&self, square: &BoardPosition, piece_mask: Bitboard) -> Bitboard {
        self.rook_lookup.lookup(square, piece_mask)
    }

    pub fn bishop_lookup(&self, square: &BoardPosition, piece_mask: Bitboard) -> Bitboard {
        self.bishop_lookup.lookup(square, piece_mask)
    }

    pub fn queen_lookup(&self, square: &BoardPosition, piece_mask: Bitboard) -> Bitboard {
        self.rook_lookup.lookup(square, piece_mask) | self.bishop_lookup.lookup(square, piece_mask)
    }
}

fn for_all_squares(function: fn(BoardPosition) -> Bitboard) -> [Bitboard; 64] {
    let mut masks = [Bitboard::empty(); 64];
    for i in 0..64 {
        masks[i as usize] = function(i.into());
    }
    masks
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PawnLookup {
    white_move_masks: [Bitboard; 64],
    white_attack_masks: [Bitboard; 64],
    black_move_masks: [Bitboard; 64],
    black_attack_masks: [Bitboard; 64],
}

impl PawnLookup {
    pub fn generate() -> Self {
        Self {
            white_move_masks: for_all_squares(|sq| pawn_move_mask(sq, PieceColor::White)),
            white_attack_masks: for_all_squares(|sq| pawn_attack_mask(sq, PieceColor::White)),
            black_move_masks: for_all_squares(|sq| pawn_move_mask(sq, PieceColor::Black)),
            black_attack_masks: for_all_squares(|sq| pawn_attack_mask(sq, PieceColor::Black)),
        }
    }

    pub fn lookup(
        &self,
        square: &BoardPosition,
        color: PieceColor,
        piece_mask: Bitboard,
        enemy_mask: Bitboard,
    ) -> Bitboard {
        let mut board = Bitboard::empty();

        let c_rank = Bitboard::from_value(0xff0000);
        let f_rank = Bitboard::from_value(0xff0000000000);

        match color {
            PieceColor::White => {
                board |= self.white_move_masks[square.index() as usize]
                    & !piece_mask
                    & !(piece_mask & c_rank).shift(1, 0);
                board |= self.white_attack_masks[square.index() as usize] & enemy_mask;
                // print_bitboard(self.white_attack_masks[square.index() as usize]);
            }
            PieceColor::Black => {
                board |= self.black_move_masks[square.index() as usize]
                    & !piece_mask
                    & !(piece_mask & f_rank).shift(-1, 0);
                board |= self.black_attack_masks[square.index() as usize] & enemy_mask;
                // print_bitboard(self.black_attack_masks[square.index() as usize]);
            }
        }

        board
    }

    pub fn attacks(&self, square: &BoardPosition, color: PieceColor) -> Bitboard {
        match color {
            PieceColor::White => self.white_attack_masks[square.index() as usize],
            PieceColor::Black => self.black_attack_masks[square.index() as usize],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JumpingPieceLookup {
    masks: [Bitboard; 64],
}

impl JumpingPieceLookup {
    pub fn generate_knight() -> Self {
        Self {
            masks: for_all_squares(knight_move_mask),
        }
    }

    pub fn generate_king() -> Self {
        Self {
            masks: for_all_squares(king_move_mask),
        }
    }

    pub fn lookup(&self, square: &BoardPosition) -> Bitboard {
        self.masks[square.index() as usize]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MagicLookup {
    pub(crate) magic: u64,
    pub(crate) length: u8,
    pub(crate) offset: u64,
}

impl MagicLookup {
    pub fn get_index(&self, mask: Bitboard) -> usize {
        ((mask.inner().wrapping_mul(self.magic) >> (64 - self.length)) + self.offset) as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SlidingPieceLookup<const N: usize> {
    magic_numbers: [MagicLookup; 64],
    masks: [Bitboard; 64],
    lookup_table: Box<[Option<Bitboard>; N]>,
}

impl<const N: usize> SlidingPieceLookup<N> {
    pub fn generate_rook() -> Self {
        let occupancy_masks = for_all_squares(rook_occupancy_mask);
        let mut lookup_table = Box::new([None; N]);

        for i in 0..64 {
            let occupancy_mask = occupancy_masks[i];
            let magic = &ROOK_MAGICS[i];

            for obstruction_mask in generate_obstruction_maps(occupancy_mask) {
                let move_mask = rook_move_mask((i as u8).into(), obstruction_mask);
                let index = magic.get_index(obstruction_mask);
                lookup_table[index] = Some(move_mask);
            }
        }

        Self {
            magic_numbers: ROOK_MAGICS,
            masks: occupancy_masks,
            lookup_table,
        }
    }

    pub fn generate_bishop() -> Self {
        let occupancy_masks = for_all_squares(bishop_occupancy_mask);
        let mut lookup_table = Box::new([None; N]);

        for i in 0..64 {
            let occupancy_mask = occupancy_masks[i];
            let magic = &BISHOP_MAGICS[i];

            for obstruction_mask in generate_obstruction_maps(occupancy_mask) {
                let move_mask = bishop_move_mask((i as u8).into(), obstruction_mask);
                let index = magic.get_index(obstruction_mask);
                lookup_table[index] = Some(move_mask);
            }
        }

        Self {
            magic_numbers: BISHOP_MAGICS,
            masks: occupancy_masks,
            lookup_table,
        }
    }

    pub fn lookup(&self, square: &BoardPosition, piece_mask: Bitboard) -> Bitboard {
        let masked = piece_mask & self.masks[square.index() as usize];
        let magic = &self.magic_numbers[square.index() as usize];
        let index = magic.get_index(masked);
        self.lookup_table[index].unwrap()
    }
}
