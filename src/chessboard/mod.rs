mod errors;
pub mod puzzles;

pub mod chess_game {
    use std::{ops::Index, usize};

    use tokio::sync::mpsc;

    use crate::{
        chessboard::errors::chess_errors::{ErrorType, MoveError},
        logger::Logger,
    };

    use super::errors::chess_errors;

    /// Piece color enum
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Color {
        White,
        Black,
    }

    impl Color {
        /// takes a square coordinates and returns the square color
        pub fn get(x: i8, y: i8) -> Color {
            let mut square_color: Color = Color::Black;
            if (x + y) % 2 == 0 {
                square_color = Color::White
            }

            return square_color;
        }

        /// Fen notation piece color
        pub fn from_sym(sym: char) -> Color {
            if sym.is_uppercase() {
                return Color::White;
            }

            return Color::Black;
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum PieceType {
        Pawn,
        Rook,
        Bishop,
        Knight,
        King,
        Queen,
    }

    impl PieceType {
        /// Returns a new Piece type based on the char
        ///
        /// # None
        ///
        /// returns none if symbol is not matching any
        /// piece type
        pub fn new(sym: char) -> Option<PieceType> {
            return match sym {
                'N' | 'n' => Some(Self::Knight),
                'B' | 'b' => Some(Self::Bishop),
                'K' | 'k' => Some(Self::King),
                'R' | 'r' => Some(Self::Rook),
                'Q' | 'q' => Some(Self::Queen),
                'P' | 'p' => Some(PieceType::Pawn),
                _ => None,
            };
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Piece {
        color: Color,
        piece: PieceType,

        pub x: i8,
        pub y: i8,
    }

    impl Piece {
        pub fn new(sym: char, x: i8, y: i8) -> Piece {
            return Piece {
                color: Color::get(x, y),
                piece: PieceType::new(sym).expect("this shit aint working"),
                x,
                y,
            };
        }

        pub fn print_color(&self) -> String {
            if self.color == Color::White {
                return "White".to_string();
            }

            return "Black".to_string();
        }
    }

    #[derive(Clone, Copy)]
    struct Square {
        square_color: Color,
        piece: Option<Piece>,

        x: i8,
        y: i8,
    }

    impl Square {
        fn new(piece: Option<Piece>, x: i8, y: i8) -> Self {
            let mut square_color: Color = Color::Black;
            if (x + y) % 2 == 0 {
                square_color = Color::White
            }

            Self {
                square_color,
                piece,
                x,
                y,
            }
        }

        /// Checks if a square is ocupied by a piece
        /// if piece is there Some(piece) is returned otherwise
        /// returns None
        pub fn has_piece(&self) -> Option<Piece> {
            if self.piece.is_some() {
                return self.piece;
            }

            return None;
        }

        fn print_square(&self) {
            if self.piece.is_some() {
                match self.piece.unwrap().piece {
                    PieceType::Pawn => print!("P"),
                    PieceType::Rook => print!("R"),
                    PieceType::Bishop => print!("B"),
                    PieceType::Knight => print!("N"),
                    PieceType::King => print!("K"),
                    PieceType::Queen => print!("Q"),
                }
            } else {
                print!("x");
            }
        }

        /// adds a piece to a square (force place)
        /// if piece is here original piece is replaced
        fn add_piece(&mut self, piece: Piece) {
            self.piece = Option::Some(piece);
        }
    }

    /// enum regarding the curent mode of play
    pub enum TrainingMode {
        Game,
        OpeningPractice(Vec<String>), // Expected sequence
        Puzzle {
            fen: String,
            solution: Vec<String>,
            current_step: usize,
        },
        Hinting,
    }

    pub struct ChessBoard {
        squares: [[Square; 8]; 8],
        remaining_pieces: Vec<Piece>,
        sender: mpsc::Sender<String>,
        mode: TrainingMode,
        curent_step: usize,
    }

    impl ChessBoard {
        pub fn new(sender: mpsc::Sender<String>, mode: TrainingMode, step: usize) -> ChessBoard {
            let mut squares: [[Square; 8]; 8] = [[Square::new(None, 0, 0); 8]; 8];

            for x in 0..8 {
                for y in 0..8 {
                    squares[x][y] = Square::new(None, x as i8, y as i8);
                }
            }

            return ChessBoard {
                sender,
                squares,
                remaining_pieces: vec![],
                mode,
                curent_step: step,
            };
        }

        /// Inits the chessboard in a puzzle mode
        pub fn PuzzleMode(
            sender: mpsc::Sender<String>,
            mode: TrainingMode,
            fen: String,
            step: usize,
        ) -> ChessBoard {
            let board = ChessBoard::from_fen(fen, sender, mode, step);
            board
        }

        /// Abstraction over square haspiece
        /// checking if a square is ocupied
        pub fn has_piece(&self, rank: i8, file: i8) -> Option<Piece> {
            return self.squares[rank as usize][file as usize].has_piece();
        }

        /// Takes a fen notation string and returns a chessboard with the
        /// specified position
        ///
        /// # Example
        ///
        /// ```rs
        ///     let mut chess = chessboard::chess_game::ChessBoard::FromFEN(
        ///         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), // fen string
        ///         tx, // mpsc sender: to forward moves to server
        ///     );
        ///```
        ///
        ///
        pub fn from_fen(
            fen: String,
            sender: mpsc::Sender<String>,
            mode: TrainingMode,
            step: usize,
        ) -> ChessBoard {
            let mut file = 0;
            let mut rank = 7;

            let mut board = ChessBoard::new(sender, mode, step);

            for x in fen.chars() {
                // println!("{}|{}|{}", x, file, rank);
                if x == '/' {
                    file = 0;
                    rank -= 1;
                } else {
                    if x.is_numeric() {
                        file += x.to_string().parse::<i8>().unwrap();
                    } else {
                        if file == 8 {
                            return board;
                        }
                        board.squares[rank as usize][file as usize]
                            .add_piece(Piece::new(x, file as i8, rank as i8));

                        file += 1;
                    }
                }
            }

            board
        }

        /// Sends the End of game signal
        pub async fn EndGame(&mut self) {
            self.sender.send(format!("{}\n", "END")).await.unwrap();
            println!("Send the end of game signal")
        }

        /// Prints the board to the terminal [`ChessBoard`]..
        pub fn print_board(&self) {
            for x in 0..8 {
                for y in 0..8 {
                    self.squares[x][y].print_square();
                }
                // self.squares[x][0].print_square();
                println!()
            }

            println!()
        }

        /// Forcefully moves a piece from source to destination without any validation.
        /// Used for puzzles where correctness is pre-validated.
        async fn force_move(&mut self, mv: String) -> Result<(), chess_errors::MoveError> {
            let y_from: i8 = get_coords_from_letter(mv.as_bytes()[0] as char).unwrap();
            let x_from: i8 = 8
                - (mv.as_bytes()[1] as char)
                    .to_string()
                    .parse::<i8>()
                    .unwrap();

            let y_to: i8 = get_coords_from_letter(mv.as_bytes()[2] as char).unwrap();
            let x_to: i8 = 8
                - (mv.as_bytes()[3] as char)
                    .to_string()
                    .parse::<i8>()
                    .unwrap();

            println!(
                "Trying to move from: ({}, {}) to ({}, {})",
                x_from, y_from, x_to, y_to
            );

            println!("{:?}", self.squares[x_from as usize][y_from as usize].piece);
            if let Some(piece) = self.has_piece(x_from, y_from) {
                self.squares[x_to as usize][y_to as usize].add_piece(piece);
                self.squares[x_from as usize][y_from as usize].piece = None;
                let _ = self.sender.send(format!("{}\n", mv)).await;
            } else {
                return Err(MoveError {
                    error_type: ErrorType::InvalidMove,
                });
            }

            Ok(())
        }

        /// makes a move based on the gamemode
        ///
        /// for puzzles it checks if the answer is correct and makes a move
        /// for normal games makes a move and nothing more
        ///
        /// and everything else is not yet made so throws a todo!
        pub async fn Move(&mut self, mv: String) -> Result<(), chess_errors::MoveError> {
            match &self.mode {
                TrainingMode::Game => self.MoveInGame(mv).await,
                TrainingMode::OpeningPractice(items) => todo!(),
                TrainingMode::Puzzle {
                    fen: _,
                    solution,
                    current_step,
                } => {
                    if mv == *solution[*current_step] {
                        self.curent_step += 1;
                        self.force_move(mv).await
                    } else {
                        return Err(chess_errors::MoveError {
                            error_type: chess_errors::ErrorType::WrongAnswearPuzzle,
                        });
                    }
                }
                TrainingMode::Hinting => todo!(),
            }
        }

        /// Checks if a move is possible and if yes makes it
        /// the move structure is parsed from a simple notation
        /// (from rank, from file) (to rank, to file) for example "E2E4"
        ///
        /// Example:
        ///
        /// ```ignore
        /// let mv_result = chessboard.move("E2E4".to_string());
        /// ```
        /// # Errors
        ///
        /// This function will return an error if the move form is invalid or if the move itself is
        /// invalid/imposible
        pub async fn MoveInGame(&mut self, mv: String) -> Result<(), chess_errors::MoveError> {
            if mv.len() < 3 {
                return Err(MoveError {
                    error_type: ErrorType::InvalidMoveStructure,
                });
            }

            let y_from: i8 = get_coords_from_letter(mv.as_bytes()[0] as char).unwrap();
            let x_from: i8 = 8
                - (mv.as_bytes()[1] as char)
                    .to_string()
                    .parse::<i8>()
                    .unwrap();

            let y_to: i8 = get_coords_from_letter(mv.as_bytes()[2] as char).unwrap();
            let x_to: i8 = 8
                - (mv.as_bytes()[3] as char)
                    .to_string()
                    .parse::<i8>()
                    .unwrap();

            // debug info about the move
            // TODO: later add a debug print module
            let mut logger = Logger;

            logger.i(format!("moving from {}, {} to {}, {}", x_from, x_to, y_from, y_to).as_str());

            if let Some(piece) = self.has_piece(x_from, y_from) {
                // normal move without any checking yet and no capture
                if self.has_piece(x_to, y_to).is_none() {
                    if self.is_move_valid(
                        piece.piece,
                        (y_from as i32, x_from as i32),
                        (y_to as i32, x_to as i32),
                        piece.color,
                    ) {
                        logger.i("this move is valid");
                        let _ = self.sender.send(format!("{}\n", mv)).await;
                        // add the piece to the designated square
                        let mut moved_piece = piece;
                        moved_piece.x = x_to;
                        moved_piece.y = y_to;
                        self.squares[x_to as usize][y_to as usize].add_piece(moved_piece);

                        // substract the piece from the previous square
                        self.squares[x_from as usize][y_from as usize].piece = None;
                    } else {
                        return Err(MoveError {
                            error_type: ErrorType::InvalidMove,
                        });
                    }
                }
            }

            return Ok(());
        }

        /// checks if a move is valid
        /// returns bool
        fn is_move_valid(
            &self,
            piece: PieceType,
            from: (i32, i32),
            to: (i32, i32),
            color: Color,
        ) -> bool {
            let (fx, fy) = from;
            let (tx, ty) = to;
            let dx = tx - fx;
            let dy = ty - fy;

            match piece {
                PieceType::Pawn => {
                    let direction = match color {
                        Color::White => -1,
                        Color::Black => 1,
                    };
                    let start_row = match color {
                        Color::White => 6,
                        Color::Black => 1,
                    };

                    // Move forward
                    if dx == 0 && dy == direction {
                        return true;
                    }

                    // Double move from starting row
                    if dx == 0 && fy == start_row && dy == 2 * direction {
                        return true;
                    }

                    // Diagonal capture
                    if dx.abs() == 1 && dy == direction {
                        return true;
                    }

                    false
                }

                PieceType::Knight => {
                    (dx.abs() == 2 && dy.abs() == 1) || (dx.abs() == 1 && dy.abs() == 2)
                }

                PieceType::Bishop => dx.abs() == dy.abs(),

                PieceType::Rook => dx == 0 || dy == 0,

                PieceType::Queen => dx == 0 || dy == 0 || dx.abs() == dy.abs(),

                PieceType::King => dx.abs().max(dy.abs()) == 1,
            }
        }
    }

    pub fn get_coords_from_letter(mv: char) -> Option<i8> {
        return match mv {
            'A' => Some(0),
            'B' => Some(1),
            'C' => Some(2),
            'D' => Some(3),
            'E' => Some(4),
            'F' => Some(5),
            'G' => Some(6),
            'H' => Some(7),
            _ => None,
        };
    }
}
