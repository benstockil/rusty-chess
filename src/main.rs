pub mod bitboards;
pub mod board;
pub mod castle;
pub mod lookup;
pub mod magics;
pub mod masks;
pub mod movement;
pub mod piece;
pub mod position;
pub mod search;
pub mod transposition;

use crate::board::FastBoard;
use crate::lookup::LookupTables;
use crate::movement::{CastleDirection, Move, Promotion};
use crate::piece::PieceColor;
use crate::position::BoardPosition;

use anyhow::bail;
use dialoguer::{theme::ColorfulTheme, Input};
use search::{EndState, MoveEngine};
use std::time::Duration;

fn parse_move(input: &str) -> anyhow::Result<BoardPosition> {
    let mut chars = input.chars();
    let file = chars.next().unwrap() as u32 - 'a' as u32;
    let rank = chars.next().unwrap().to_digit(10).unwrap() - 1;
    Ok(BoardPosition::from_rank_file(rank as u8, file as u8))
}

fn main() -> anyhow::Result<()> {
    let mut move_engine = MoveEngine::new();

    let fen: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Initial FEN (empty for new game)")
        .allow_empty(true)
        .interact_text()?;

    let mut board = if fen.is_empty() {
        FastBoard::initial()
    } else {
        FastBoard::from_fen(&fen)?
    };

    // print!("User color [W/b]: ");
    let user_color = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("User color")
        .default("white".to_string())
        .interact_text()?;

    let user_color = match &*user_color {
        "white" | "w" | "" => PieceColor::White,
        "black" | "b" => PieceColor::Black,
        _ => panic!(),
    };

    if user_color == board.next_to_move {
        print_board(&board);
        if check_end_state(&mut board, &mut move_engine)? {
            return Ok(());
        }

        user_move(&mut board)?;
    }

    loop {
        print_board(&board);
        if check_end_state(&mut board, &mut move_engine)? {
            return Ok(());
        }

        // COMPUTER MOVE
        computer_move(&mut board, &mut move_engine)?;

        print_board(&board);
        if check_end_state(&mut board, &mut move_engine)? {
            return Ok(());
        }

        // USER MOVE
        user_move(&mut board)?;
    }
}

enum UserAction {
    Move(Move),
    Unmake,
}

fn parse_action(input: &str) -> anyhow::Result<UserAction> {
    let mut words = input.trim().split(" ");

    let first = words.next().unwrap();

    match first {
        "castle" => {
            let second = words.next().unwrap();
            let castle = Move::Castle(match second {
                "queenside" => CastleDirection::QueenSide,
                "kingside" => CastleDirection::KingSide,
                _ => bail!("unknown castling direction"),
            });
            Ok(UserAction::Move(castle))
        }

        "unmake" => Ok(UserAction::Unmake),

        _ => {
            let second = words.next().unwrap();
            let from = parse_move(first)?;
            let to = parse_move(second)?;

            let promotion = words
                .next()
                .map(|third| {
                    Ok(match third {
                        "queen" => Promotion::Queen,
                        "rook" => Promotion::Rook,
                        "knight" => Promotion::Knight,
                        "bishop" => Promotion::Bishop,
                        _ => bail!("unknown promotion piece"),
                    })
                })
                .transpose()?;

            let user_move = Move::Direct {
                from,
                to,
                promotion,
            };

            Ok(UserAction::Move(user_move))
        }
    }
}

fn check_end_state(board: &mut FastBoard, engine: &mut MoveEngine) -> anyhow::Result<bool> {
    let Some(end_state) = engine.get_end_state(board) else {
        return Ok(false);
    };

    match end_state {
        EndState::Checkmate => {
            println!("Checkmate! {:?} wins.", board.next_to_move.other())
        }
        EndState::Stalemate => println!("Stalemate!"),
        EndState::ThreeFoldRepetiiton => println!("Draw (3-fold)!"),
    }

    Ok(true)
}

fn computer_move(board: &mut FastBoard, engine: &mut MoveEngine) -> anyhow::Result<()> {
    println!("Searching for best move...");
    let next_move = engine.iterative_deepening(board, Duration::from_secs(5));

    println!("Best move: {}", next_move);
    board.make_move(next_move)?;

    Ok(())
}

fn user_move(board: &mut FastBoard) -> anyhow::Result<()> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your move")
        .interact_text()
        .unwrap();

    match parse_action(&input)? {
        UserAction::Move(board_move) => {
            board.make_move(board_move).unwrap();
        }
        UserAction::Unmake => {
            board.unmake_last_move();
            board.unmake_last_move();
        }
    }

    Ok(())
}

pub(crate) fn print_board(board: &FastBoard) {
    println!("Board State: {}\n", board.to_fen());

    for i in 0..8 {
        print!(" {}   ", 8 - i);
        for j in 0..8 {
            let square_char = match board.mailbox.get(&((7 - i), j).into()) {
                None => '.',
                Some(piece) => piece.to_char(),
            };
            print!("{} ", square_char);
        }
        println!();
    }
    println!("\n     a b c d e f g h\n");
}

// fn main() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     terminal.clear()?;
//
//     let mut app = ChessApp {
//         board: FastBoard::initial(),
//         exit: false,
//     };
//
//     app.run(&mut terminal)?;
//
//     ratatui::restore();
//
//     Ok(())
// }

// struct ChessApp {
//     board: FastBoard,
//     exit: bool,
// }
//
// pub struct DisplayBoard<'a> {
//     pieces: &'a Mailbox,
// }
//
// fn render_grid_square<'a>(piece: Option<Piece>, row: u8, column: u8) -> Paragraph<'a> {
//     let span = match piece {
//         None => "".into(),
//         Some(piece) => {
//             let char = match piece.kind {
//                 PieceType::King => "K",
//                 PieceType::Queen => "Q",
//                 PieceType::Rook => "R",
//                 PieceType::Bishop => "B",
//                 PieceType::Knight => "C",
//                 PieceType::Pawn => "P",
//             };
//
//             match piece.color {
//                 PieceColor::White => char.green(),
//                 PieceColor::Black => char.blue(),
//             }
//             .bold()
//         }
//     };
//
//     let bg = match (row + column) % 2 {
//         0 => Color::Rgb(60, 56, 54),
//         1 => Color::Rgb(21, 21, 20),
//         _ => unreachable!(),
//     };
//
//     let block = Block::new().style(Style::new().bg(bg));
//     // .padding(Padding::symmetric(2, 1));
//
//     Paragraph::new(span).block(block).centered()
// }
//
// impl Widget for DisplayBoard<'_> {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let grid_layout = Layout::vertical([Constraint::Fill(1); 8]).split(area);
//
//         for row in 0..8 {
//             let row_layout =
//                 Layout::horizontal([Constraint::Fill(1); 8]).split(grid_layout[row as usize]);
//
//             for column in 0..8 {
//                 let piece = self.pieces.get(&(row, column).into());
//                 let square = render_grid_square(piece, row, column);
//                 square.render(row_layout[column as usize], buf);
//             }
//         }
//     }
// }
//
// impl ChessApp {
//     /// runs the application's main loop until the user quits
//     pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//         while !self.exit {
//             terminal.draw(|frame| self.draw(frame))?;
//             self.handle_events()?;
//         }
//         Ok(())
//     }
//
//     fn draw(&self, frame: &mut Frame) {
//         frame.render_widget(self, frame.area());
//     }
//
//     fn handle_events(&mut self) -> io::Result<()> {
//         match event::read()? {
//             // it's important to check that the event is a key press event as
//             // crossterm also emits key release and repeat events on Windows.
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
//                 self.handle_key_event(key_event)
//             }
//             _ => {}
//         };
//         Ok(())
//     }
//
//     fn handle_key_event(&mut self, key_event: KeyEvent) {
//         match key_event.code {
//             KeyCode::Char('q') => self.exit(),
//             // KeyCode::Left => self.decrement_counter(),
//             // KeyCode::Right => self.increment_counter(),
//             _ => {}
//         }
//     }
//
//     fn exit(&mut self) {
//         self.exit = true;
//     }
// }
//
// impl Widget for &ChessApp {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let title = Title::from(" Chess Bot ".bold());
//         let instructions = Line::from(vec![
//             " Decrement ".into(),
//             "<Left>".blue().bold(),
//             " Increment ".into(),
//             "<Right>".blue().bold(),
//             " Quit ".into(),
//             "<Q> ".blue().bold(),
//         ]);
//
//         let block = Block::bordered()
//             .title(title)
//             .title_bottom(instructions)
//             .border_set(border::THICK);
//
//         // Setup interface layouts
//         let [main_area, sidebar_area] =
//             Layout::horizontal([Constraint::Fill(1), Constraint::Length(50)]).areas(area);
//
//         let [game_area, below] =
//             Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);
//
//         let [stats_area, log_area, controls_area] = Layout::vertical([
//             Constraint::Length(30),
//             Constraint::Fill(1),
//             Constraint::Fill(1),
//         ])
//         .areas(sidebar_area);
//         let board_height = game_area.height;
//         let [board_area, history_area] =
//             Layout::horizontal([Constraint::Length(board_height * 2), Constraint::Fill(1)])
//                 .areas(game_area);
//
//         // Style container blocks for interface elements
//         let stats_block = Block::bordered().title(" Stats ".bold());
//         let log_block = Block::bordered().title(" Log ".bold());
//         let controls_block = Block::bordered().title(" Controls ".bold());
//         let board_block = Block::bordered().title(" Board ".bold());
//         let history_block = Block::bordered().title(" Move History ".bold());
//
//         // Create and render some dummy interface components
//         Paragraph::new("These are some stats!")
//             .block(stats_block)
//             .render(stats_area, buf);
//
//         Paragraph::new("Log").block(log_block).render(log_area, buf);
//
//         Paragraph::new("Controls")
//             .block(controls_block)
//             .render(controls_area, buf);
//
//         Paragraph::new("Historyyyyy")
//             .block(history_block)
//             .render(history_area, buf);
//
//         let board = self.board.get_display_board();
//         board.render(board_block.inner(board_area), buf);
//         board_block.render(board_area, buf);
//         // board.render(layout[0], buf);
//     }
// }
