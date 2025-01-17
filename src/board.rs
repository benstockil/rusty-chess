use std::collections::HashMap;

use anyhow::bail;

use crate::bitboards::{Bitboard, Bitboards};
use crate::castle::{CastlingRights, PlayerCastlingRights};
use crate::lookup::LOOKUP;
use crate::movement::{CastleDirection, Move, PastMove, Promotion};
use crate::piece::{Piece, PieceColor, PieceType};
use crate::position::BoardPosition;
use crate::transposition::ZobristKey;

#[derive(Debug, PartialEq, Clone)]
pub struct Mailbox {
    pieces: [Option<Piece>; 64],
}

impl Mailbox {
    fn new() -> Self {
        Self { pieces: [None; 64] }
    }

    pub fn get(&self, position: &BoardPosition) -> Option<Piece> {
        self.pieces[position.index() as usize]
    }

    pub fn insert(&mut self, position: BoardPosition, piece: Piece) -> Option<Piece> {
        let old_piece = self.get(&position);
        self.pieces[position.index() as usize] = Some(piece);
        old_piece
    }

    pub fn remove(&mut self, position: &BoardPosition) -> Option<Piece> {
        let old_piece = self.get(position);
        self.pieces[position.index() as usize] = None;
        old_piece
    }

    pub fn iter(&self) -> impl Iterator<Item = (BoardPosition, Piece)> + use<'_> {
        self.pieces
            .iter()
            .enumerate()
            .filter_map(|(i, piece)| piece.map(|p| (BoardPosition::from_index(i as u8), p)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FastBoard {
    pub(crate) mailbox: Mailbox,
    pub(crate) move_list: Vec<PastMove>,
    pub(crate) zobrist_key: ZobristKey,
    pub(crate) next_to_move: PieceColor,
    pub(crate) repetitions: HashMap<ZobristKey, u8>,
    bitboards: Bitboards,
    en_passant: Option<u8>,
    castling_rights: CastlingRights,
    halfmoves: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum MoveError {
    #[error("Piece at {origin:?} not found")]
    PieceNotFound { origin: BoardPosition },
    #[error("Piece at {destination:?} is {captured_color:?}'s own piece")]
    CapturesOwnPiece {
        destination: BoardPosition,
        captured_color: PieceColor,
    },
    #[error("Piece at {origin:?} is the opponent's ({moved_color:?}) piece.")]
    MovesOpponentsPiece {
        origin: BoardPosition,
        moved_color: PieceColor,
    },
    #[error("Move is illegal.")]
    IllegalMove,
}

impl FastBoard {
    pub fn empty() -> Self {
        Self {
            mailbox: Mailbox::new(),
            bitboards: Bitboards::empty(),
            move_list: Vec::new(),
            next_to_move: PieceColor::White,
            en_passant: None,
            castling_rights: CastlingRights::default(),
            zobrist_key: ZobristKey::new(),
            halfmoves: 0,
            repetitions: HashMap::new(),
            // move_generator: LookupTables::new(),
        }
    }

    pub fn from_fen(fen: &str) -> anyhow::Result<Self> {
        let fields: Vec<_> = fen.split(" ").collect();
        assert!(fields.len() == 6, "Incorrect number of FEN fields");
        let ranks: Vec<_> = fields.get(0).unwrap().split("/").collect();
        assert!(ranks.len() == 8, "Incorrect number of FEN ranks");

        let mut board = Self::empty();

        for (rank, rank_fen) in ranks.iter().enumerate() {
            let mut file = 0;
            for char in rank_fen.chars() {
                if let Some(digit) = char.to_digit(10) {
                    file += digit;
                } else {
                    let piece = match char.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => bail!("Invalid FEN piece"),
                    };
                    let color = match char.is_uppercase() {
                        true => PieceColor::White,
                        false => PieceColor::Black,
                    };

                    board.place_piece(
                        BoardPosition::from_rank_file(7 - rank as u8, file as u8),
                        Piece::new(color, piece),
                    );

                    file += 1;
                }
            }
        }

        board.next_to_move = match fields[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => bail!("Invalid FEN next piece color"),
        };

        let rights = fields[2];
        board.castling_rights = CastlingRights {
            white: PlayerCastlingRights {
                queenside: rights.contains('Q'),
                kingside: rights.contains('K'),
            },
            black: PlayerCastlingRights {
                queenside: rights.contains('q'),
                kingside: rights.contains('k'),
            },
        };

        board.en_passant = if fields[3] != "-" {
            let file_fen = fields[3].chars().next().unwrap();
            Some((file_fen as u32 - 'a' as u32) as u8)
        } else {
            None
        };

        board.halfmoves = fields[4].parse().unwrap();

        Ok(board)
    }

    pub fn initial() -> Self {
        let mut board =
            Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        board.repetitions.insert(board.zobrist_key, 1);

        board
    }

    fn place_piece(&mut self, position: BoardPosition, piece: Piece) -> Option<Piece> {
        // Insert the new piece into the mailbox, replacing the old one
        let replaced = self.mailbox.insert(position, piece);

        // Remove the replaced piece from the bitboards before inserting the new one
        if let Some(replaced) = replaced {
            self.bitboards.remove(position.index(), replaced);
            self.zobrist_key.toggle_piece(&replaced, &position);
        }

        // Insert the new piece into the bitboards
        self.bitboards.insert(position.index(), piece);
        self.zobrist_key.toggle_piece(&piece, &position);

        replaced
    }

    fn remove_piece(&mut self, position: &BoardPosition) -> Option<Piece> {
        let piece = self.mailbox.remove(position);
        if let Some(piece) = piece {
            self.bitboards.remove(position.index(), piece);
            self.zobrist_key.toggle_piece(&piece, position);
            // print_bitboard(self.bitboards.get_color(piece.color));
        }

        piece
    }

    fn home_row(player: PieceColor) -> u8 {
        match player {
            PieceColor::White => 0,
            PieceColor::Black => 7,
        }
    }

    fn perform_castle(&mut self, direction: CastleDirection) {
        let home_row = Self::home_row(self.next_to_move);

        let (king_dest, rook_orig, rook_dest) = match direction {
            CastleDirection::QueenSide => (2, 0, 3),
            CastleDirection::KingSide => (6, 7, 5),
        };

        // Move the king
        let king = self.remove_piece(&(home_row, 4).into()).unwrap();
        self.place_piece((home_row, king_dest).into(), king);

        // Move the rook
        let rook = self.remove_piece(&(home_row, rook_orig).into()).unwrap();
        self.place_piece((home_row, rook_dest).into(), rook);

        // print_board(self);
    }

    fn revert_castle(&mut self, direction: CastleDirection) {
        let home_row = Self::home_row(self.next_to_move.other());

        let (king_dest, rook_orig, rook_dest) = match direction {
            CastleDirection::QueenSide => (2, 0, 3),
            CastleDirection::KingSide => (6, 7, 5),
        };

        // Move the king
        let king = self.remove_piece(&(home_row, king_dest).into()).unwrap();
        self.place_piece((home_row, 4).into(), king);

        // Move the rook
        let rook = self.remove_piece(&(home_row, rook_dest).into()).unwrap();
        self.place_piece((home_row, rook_orig).into(), rook);
        // print_board(self);
    }

    fn update_castling_rights(
        &mut self,
        origin: &BoardPosition,
        destination: &BoardPosition,
    ) -> Option<CastlingRights> {
        let previous_castle = self.castling_rights;
        let mut has_changed = false;

        let home_row = Self::home_row(self.next_to_move);
        let player_castling = self.castling_rights.get_mut(self.next_to_move);

        if origin.rank() == home_row {
            if player_castling.queenside && origin.file() == 0 {
                has_changed = true;
                player_castling.forbid_queenside();
            }

            if player_castling.kingside && origin.file() == 7 {
                has_changed = true;
                player_castling.forbid_kingside();
            }

            if (player_castling.queenside || player_castling.kingside) && origin.file() == 4 {
                has_changed = true;
                player_castling.forbid_all();
            }
        }

        let enemy_home_row = Self::home_row(self.next_to_move.other());
        let enemy_castling = self.castling_rights.get_mut(self.next_to_move.other());

        if destination.rank == enemy_home_row {
            if enemy_castling.queenside && destination.file() == 0 {
                has_changed = true;
                enemy_castling.forbid_queenside();
            }

            if enemy_castling.kingside && destination.file() == 7 {
                has_changed = true;
                enemy_castling.forbid_kingside();
            }
        }

        if has_changed {
            self.zobrist_key.toggle_castling_rights(&previous_castle);
            self.zobrist_key
                .toggle_castling_rights(&self.castling_rights);

            Some(previous_castle)
        } else {
            None
        }
    }

    fn set_en_passant(&mut self, file: Option<u8>) {
        if let Some(file) = self.en_passant {
            self.zobrist_key.toggle_en_passant(file);
        }

        if let Some(file) = file {
            self.zobrist_key.toggle_en_passant(file);
        }

        self.en_passant = file;
    }

    fn update_en_passant(
        &mut self,
        moved_piece: &Piece,
        origin: &BoardPosition,
        destination: &BoardPosition,
    ) -> Option<u8> {
        let previous_en_passant = self.en_passant;

        if let Some(file) = previous_en_passant {
            self.en_passant = None;
            self.zobrist_key.toggle_en_passant(file);
        }

        let (pawn_row, jump_row) = match self.next_to_move {
            PieceColor::White => (1, 3),
            PieceColor::Black => (6, 4),
        };

        if matches!(moved_piece.kind, PieceType::Pawn)
            && origin.rank() == pawn_row
            && destination.rank() == jump_row
        {
            let file = origin.file();
            self.set_en_passant(Some(file));
        }

        previous_en_passant
    }

    fn toggle_next_player(&mut self) {
        self.next_to_move = self.next_to_move.other();
        self.zobrist_key.toggle_player();
    }

    pub fn make_move(&mut self, board_move: Move) -> Result<(), MoveError> {
        // println!("MAKING MOVE {:?}: {:?}", self.next_to_move, board_move);

        let past_move = match board_move {
            Move::Direct {
                from,
                to,
                promotion,
            } => {
                // TODO: Fix slightly hacky clone to get around borrow checker
                let Some(moved_piece) = self.mailbox.get(&from).map(|p| p.to_owned()) else {
                    return Err(MoveError::PieceNotFound { origin: from });
                };

                if moved_piece.color != self.next_to_move {
                    return Err(MoveError::MovesOpponentsPiece {
                        origin: from,
                        moved_color: moved_piece.color,
                    });
                }

                let captured = self.mailbox.get(&to);
                let captured_kind = captured.map(|p| p.kind);
                if let Some(captured_piece) = captured {
                    if captured_piece.color == self.next_to_move {
                        return Err(MoveError::CapturesOwnPiece {
                            destination: to,
                            captured_color: captured_piece.color,
                        });
                    }
                }

                let previous_en_passant = self.update_en_passant(&moved_piece, &from, &to);

                let previous_castle = self.update_castling_rights(&from, &to);

                let past_move = PastMove::new(
                    board_move,
                    captured_kind,
                    previous_castle,
                    previous_en_passant,
                );

                // Move the piece
                let mut piece = self.remove_piece(&from).unwrap();
                // If the piece has been promoted, change its type.
                if let Some(promotion) = promotion {
                    piece.kind = promotion.piece_type();
                };
                self.place_piece(to, piece);

                past_move
            }

            Move::Castle(direction) => {
                if self.is_in_check(self.next_to_move) {
                    return Err(MoveError::IllegalMove);
                }

                let rank = Self::home_row(self.next_to_move);
                let mid_files = match direction {
                    CastleDirection::QueenSide => [1, 2, 3].iter(),
                    CastleDirection::KingSide => [5, 6].iter(),
                };

                if let Err(MoveError::IllegalMove) = mid_files
                    .map(|file| {
                        let mid_move = Move::Direct {
                            from: BoardPosition::from_rank_file(rank, 4),
                            to: BoardPosition::from_rank_file(rank, *file),
                            promotion: None,
                        };
                        self.make_move(mid_move)?;
                        self.unmake_last_move();
                        Ok(())
                    })
                    .collect::<Result<Vec<()>, MoveError>>()
                {
                    return Err(MoveError::IllegalMove);
                }

                // Perform the castle move
                self.perform_castle(direction);

                // Record the move made (clones current castle options)
                let past_move = PastMove::new(
                    board_move,
                    None,
                    Some(self.castling_rights),
                    self.en_passant,
                );

                self.set_en_passant(None);

                // Forbid all future castling (as the player has already castled).
                self.zobrist_key
                    .toggle_castling_rights(&self.castling_rights);

                let castling_rights = self.castling_rights.get_mut(self.next_to_move);
                castling_rights.forbid_all();

                self.zobrist_key
                    .toggle_castling_rights(&self.castling_rights);

                past_move
            }
        };

        self.move_list.push(past_move);
        self.toggle_next_player();
        self.halfmoves += 1;

        if self.is_in_check(self.next_to_move.other()) {
            self.unmake_last_move();
            return Err(MoveError::IllegalMove);
        }

        self.repetitions
            .entry(self.zobrist_key)
            .and_modify(|r| *r += 1)
            .or_insert(1);

        Ok(())
    }

    pub fn unmake_last_move(&mut self) {
        self.halfmoves -= 1;
        self.repetitions
            .entry(self.zobrist_key)
            .and_modify(|r| *r -= 1);

        // dbg!(&self.move_list);
        let previous_move = self.move_list.pop().unwrap();
        // println!("UNMAKING MOVE");

        if let Some(previous_rights) = previous_move.previous_castling_rights {
            self.zobrist_key
                .toggle_castling_rights(&self.castling_rights);
            self.zobrist_key.toggle_castling_rights(&previous_rights);
            self.castling_rights = previous_rights;
        }

        self.set_en_passant(previous_move.previous_en_passant);

        match previous_move.move_made {
            Move::Direct {
                from,
                to,
                promotion,
            } => {
                let mut piece = self.remove_piece(&to).unwrap();

                if promotion.is_some() {
                    piece.kind = PieceType::Pawn;
                }

                self.place_piece(from, piece);

                if let Some(captured) = previous_move.captured {
                    self.place_piece(to, Piece::new(self.next_to_move, captured));
                }
            }
            Move::Castle(direction) => {
                self.revert_castle(direction);
            }
        }

        // print_board(self);
        self.toggle_next_player();
    }

    pub fn calculate_pseudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(212);
        for (pos, piece) in self.mailbox.iter() {
            if piece.color == self.next_to_move {
                let mut piece_moves = self.calculate_pseudo_moves_for_piece(&pos, &piece);
                moves.append(&mut piece_moves);
            }
        }

        let castle_options = self.castling_rights.get(self.next_to_move);
        let (kingside_mask, queenside_mask) = match self.next_to_move {
            PieceColor::White => (Bitboard::from_value(0x60), Bitboard::from_value(0xE)),
            PieceColor::Black => (
                Bitboard::from_value(0x6000000000000000),
                Bitboard::from_value(0xE00000000000000),
            ),
        };

        if castle_options.kingside && (self.bitboards.all_pieces() & kingside_mask).is_empty() {
            moves.push(Move::Castle(CastleDirection::KingSide));
        }

        if castle_options.queenside && (self.bitboards.all_pieces() & queenside_mask).is_empty() {
            moves.push(Move::Castle(CastleDirection::QueenSide));
        }

        moves
    }

    pub fn calculate_pseudo_moves_for_piece<'a>(
        &self,
        position: &'a BoardPosition,
        piece: &'a Piece,
    ) -> Vec<Move> {
        let piece_mask = self.bitboards.all_pieces();
        let enemy_mask = self.bitboards.get_color(self.next_to_move.other()).any();
        let own_piece_mask = self.bitboards.get_color(self.next_to_move).any();

        let mut legal_square_bitboard = match piece.kind {
            PieceType::King => LOOKUP.king_lookup(position),
            PieceType::Queen => LOOKUP.queen_lookup(position, piece_mask),
            PieceType::Bishop => LOOKUP.bishop_lookup(position, piece_mask),
            PieceType::Knight => LOOKUP.knight_lookup(position),
            PieceType::Rook => LOOKUP.rook_lookup(position, piece_mask),
            PieceType::Pawn => {
                LOOKUP.pawn_lookup(position, self.next_to_move, piece_mask, enemy_mask)
            }
        };

        // print_bitboard(legal_square_bitboard);
        legal_square_bitboard &= !own_piece_mask;
        // print_bitboard(legal_square_bitboard);

        let mut moves = Vec::with_capacity(16);
        for index in legal_square_bitboard.filled_indices() {
            let dest = BoardPosition::from_index(index);
            if piece.kind == PieceType::Pawn
                && dest.rank() == Self::home_row(self.next_to_move.other())
            {
                for promotion in [
                    Promotion::Rook,
                    Promotion::Queen,
                    Promotion::Bishop,
                    Promotion::Knight,
                ] {
                    moves.push(Move::Direct {
                        from: *position,
                        to: dest,
                        promotion: Some(promotion),
                    });
                }
            } else {
                moves.push(Move::Direct {
                    from: *position,
                    to: dest,
                    promotion: None,
                });
            }
        }

        moves
    }

    #[must_use]
    pub fn is_in_check(&mut self, color: PieceColor) -> bool {
        let king_pos = self
            .mailbox
            .iter()
            .filter(|(_, piece)| piece.kind == PieceType::King && piece.color == color)
            .map(|(pos, _)| pos)
            .next()
            .unwrap();

        let mask = self.bitboards.all_pieces();
        let bitboards = self.bitboards.get_color(color.other());

        #[rustfmt::skip]
        return (LOOKUP.queen_lookup (&king_pos, mask)  & bitboards.queen).any()
            || (LOOKUP.rook_lookup  (&king_pos, mask)  & bitboards.rook).any()
            || (LOOKUP.bishop_lookup(&king_pos, mask)  & bitboards.bishop).any()
            || (LOOKUP.king_lookup  (&king_pos)        & bitboards.king).any()
            || (LOOKUP.knight_lookup(&king_pos)        & bitboards.knight).any()
            || (LOOKUP.pawn_attacks (&king_pos, color) & bitboards.pawn).any();
    }

    pub fn evaluate(&self) -> i32 {
        // A positive score favours whoever is next to move.
        // A negative score favours whoever just moved.
        let mut score = 0;

        for (position, piece) in self.mailbox.iter() {
            let piece_value = match piece.kind {
                PieceType::King => 20000,
                PieceType::Queen => 900,
                PieceType::Rook => 500,
                PieceType::Bishop => 330,
                PieceType::Knight => 320,
                PieceType::Pawn => 100,
            };

            let piece_square_table = match piece.kind {
                PieceType::King => KING_MIDGAME_PST,
                PieceType::Queen => QUEEN_PST,
                PieceType::Bishop => BISHOP_PST,
                PieceType::Knight => KNIGHT_PST,
                PieceType::Rook => ROOK_PST,
                PieceType::Pawn => PAWN_PST,
            };

            let pst_index = match piece.color {
                PieceColor::Black => position.rank * 8 + position.file,
                PieceColor::White => (7 - position.rank) * 8 + position.file,
            };

            let square_value = piece_square_table[pst_index as usize];

            let total_value = piece_value + square_value;

            if piece.color == self.next_to_move {
                score += total_value;
            } else {
                score -= total_value;
            }
        }

        score
    }

    pub fn check_board_state(&self, fen: &str) -> bool {
        let fen_board = Self::from_fen(fen).unwrap();
        self.mailbox == fen_board.mailbox && self.en_passant == fen_board.en_passant
    }

    pub(crate) fn get_repetitions(&self) -> u8 {
        *self.repetitions.get(&self.zobrist_key).unwrap_or(&0)
    }

    pub fn to_fen(&self) -> String {
        // BOARD REPRESENTATION
        let mut board = String::new();
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let piece = self.mailbox.get(&BoardPosition::from_rank_file(rank, file));
                match piece {
                    Some(piece) => {
                        if empty > 0 {
                            board.push_str(&empty.to_string());
                            empty = 0;
                        }
                        board.push(piece.to_char());
                    }
                    None => empty += 1,
                }
            }

            if empty > 0 {
                board.push_str(&empty.to_string());
            }

            if rank != 0 {
                board.push('/');
            }
        }

        // NEXT TO MOVE
        let to_move = match self.next_to_move {
            PieceColor::White => 'w',
            PieceColor::Black => 'b',
        };

        // CASTLING RIGHTS
        let mut castling = String::new();

        if self.castling_rights.white.kingside {
            castling.push('K');
        }

        if self.castling_rights.white.queenside {
            castling.push('Q');
        }

        if self.castling_rights.black.kingside {
            castling.push('k');
        }

        if self.castling_rights.black.queenside {
            castling.push('q');
        }

        // EN PASSANT
        let en_passant = match self.en_passant {
            Some(ep) => ep.to_string(),
            None => "-".into(),
        };

        // COUNTER
        let full_moves = self.halfmoves / 2;

        format!(
            "{} {} {} {} {} {}",
            board, to_move, castling, en_passant, self.halfmoves, full_moves,
        )
    }
}

#[rustfmt::skip]
const PAWN_PST: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  25,  25,  10,   5,   5,
     0,   0,   0,  20,  20,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -20, -20,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const ROOK_PST: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
const QUEEN_PST: [i32; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,   5,   5,   5,   0, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

#[rustfmt::skip]
const KING_MIDGAME_PST: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];

#[rustfmt::skip]
const KING_ENDGAME_PST: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -50, -30, -30, -30, -30, -30, -30, -50,
];

// mod tests {
//     use crate::{board::FastBoard, print_board};
//
//     #[test]
//     fn test_fen() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//         let fen_board = FastBoard::from_fen(fen).unwrap();
//         print_board(&fen_board);
//         assert_eq!(fen_board, FastBoard::initial());
//     }
// }
