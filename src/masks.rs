use crate::{bitboards::Bitboard, piece::PieceColor, position::BoardPosition};

pub fn pawn_attack_mask(position: BoardPosition, color: PieceColor) -> Bitboard {
    let pos = Bitboard::from_pos(&position);
    let mut board = Bitboard::empty();

    let rank_offset = match color {
        PieceColor::White => 1,
        PieceColor::Black => -1,
    };

    if position.file() != 0 {
        board |= pos.shift(rank_offset, -1);
    }
    if position.file() != 7 {
        board |= pos.shift(rank_offset, 1);
    }

    board
}

pub fn pawn_move_mask(position: BoardPosition, color: PieceColor) -> Bitboard {
    let pos = Bitboard::from_pos(&position);
    let mut board = Bitboard::empty();
    match color {
        PieceColor::White => {
            board |= pos.shift(1, 0);
            if position.rank() == 1 {
                board |= pos.shift(2, 0);
            }
        }
        PieceColor::Black => {
            board |= pos.shift(-1, 0);
            if position.rank() == 6 {
                board |= pos.shift(-2, 0);
            }
        }
    }
    // print_bitboard(board);
    board
}

pub fn knight_move_mask(position: BoardPosition) -> Bitboard {
    let pos = Bitboard::from_pos(&position);
    let mut board = Bitboard::empty();

    if position.file() > 1 {
        board |= pos.shift(-1, -2) | pos.shift(1, -2);
    }

    if position.file() > 0 {
        board |= pos.shift(-2, -1) | pos.shift(2, -1);
    }

    if position.file() < 7 {
        board |= pos.shift(-2, 1) | pos.shift(2, 1);
    }

    if position.file() < 6 {
        board |= pos.shift(-1, 2) | pos.shift(1, 2);
    }

    board
}

pub fn king_move_mask(position: BoardPosition) -> Bitboard {
    let pos = Bitboard::from_pos(&position);
    let mut board = Bitboard::empty();

    board |= pos.shift(-1, 0) | pos.shift(1, 0);

    if position.file() != 0 {
        board |= pos.shift(-1, -1) | pos.shift(0, -1) | pos.shift(1, -1);
    }

    if position.file() != 7 {
        board |= pos.shift(-1, 1) | pos.shift(0, 1) | pos.shift(1, 1);
    }

    board
}

pub fn bishop_occupancy_mask(position: BoardPosition) -> Bitboard {
    let mut board = Bitboard::empty();

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() + i, position.file() + i);
        board.set(dest.index(), true);

        if dest.rank() >= 6 || dest.file() >= 6 {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() + i, position.file() - i);
        board.set(dest.index(), true);

        if dest.rank() >= 6 || dest.file() <= 1 {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() - i, position.file() + i);
        board.set(dest.index(), true);

        if dest.rank() <= 1 || dest.file() >= 6 {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() - i, position.file() - i);

        board.set(dest.index(), true);

        if dest.rank() <= 1 || dest.file() <= 1 {
            break;
        }
    }

    board.set(position.index(), false);

    board
}

pub fn rook_occupancy_mask(position: BoardPosition) -> Bitboard {
    let mut board = Bitboard::empty();

    for i in 1..7 {
        board.set(position.rank() * 8 + i, true);
        board.set(i * 8 + position.file(), true);
    }

    board.set(position.index(), false);

    board
}

pub fn rook_move_mask(position: BoardPosition, piece_mask: Bitboard) -> Bitboard {
    let mut board = Bitboard::empty();

    for rank in (position.rank() + 1)..8 {
        let index = rank * 8 + position.file();
        board.set(index, true);

        if piece_mask.get(index) {
            break;
        }
    }

    for rank in (0..position.rank()).rev() {
        let index = rank * 8 + position.file();
        board.set(index, true);

        if piece_mask.get(index) {
            break;
        }
    }

    for file in (position.file() + 1)..8 {
        let index = position.rank() * 8 + file;
        board.set(index, true);

        if piece_mask.get(index) {
            break;
        }
    }

    for file in (0..position.file()).rev() {
        let index = position.rank() * 8 + file;
        board.set(index, true);

        if piece_mask.get(index) {
            break;
        }
    }

    board
}

pub fn bishop_move_mask(position: BoardPosition, mut piece_mask: Bitboard) -> Bitboard {
    let mut board = Bitboard::empty();
    // let index = position.index();

    piece_mask.set(position.index(), false);

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() + i, position.file() + i);
        board.set(dest.index(), true);

        if dest.rank() == 7 || dest.file() == 7 || piece_mask.get(dest.index()) {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() + i, position.file() - i);
        board.set(dest.index(), true);

        if dest.rank() == 7 || dest.file() == 0 || piece_mask.get(dest.index()) {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() - i, position.file() + i);
        board.set(dest.index(), true);

        if dest.rank() == 0 || dest.file() == 7 || piece_mask.get(dest.index()) {
            break;
        }
    }

    for i in 0.. {
        let dest = BoardPosition::from_rank_file(position.rank() - i, position.file() - i);

        board.set(dest.index(), true);

        if dest.rank() == 0 || dest.file() == 0 || piece_mask.get(dest.index()) {
            break;
        }
    }

    board.set(position.index(), false);

    board
}

pub fn generate_obstruction_maps(mask: Bitboard) -> Vec<Bitboard> {
    let mut boards = Vec::new();
    for permutation in 0.. {
        let mut moving_permutation = permutation;
        let mut board = Bitboard::empty();
        let mut position = BoardPosition::from_index(0);

        while moving_permutation != 0 && position.index() < 64 {
            if mask.get(position.index()) {
                board.set(position.index(), moving_permutation & 1 == 1);
                moving_permutation >>= 1;
            }
            position = BoardPosition::from_index(position.index() + 1);
        }

        boards.push(board);

        if board == mask {
            break;
        }
    }

    boards
}
