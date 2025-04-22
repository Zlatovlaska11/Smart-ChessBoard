mod errors;

pub mod chess_game {
    use std::{io, ops::Index, usize};

    use crate::chessboard::errors::chess_errors::MoveStructureError;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Color {
        White,
        Black,
    }

    impl Color {
        pub fn Get(x: i8, y: i8) -> Color {
            let mut square_color: Color = Color::Black;
            if (x + y) % 2 == 0 {
                square_color = Color::White
            }

            return square_color;
        }

        pub fn FromSym(sym: char) -> Color {
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

    #[derive(Clone, Copy)]
    struct Piece {
        color: Color,
        piece: PieceType,

        x: i8,
        y: i8,
    }

    impl Piece {
        pub fn new(sym: char, x: i8, y: i8) -> Piece {
            return Piece {
                color: Color::Get(x, y),
                piece: PieceType::new(sym).expect("this shit aint working"),
                x,
                y,
            };
        }

        pub fn printColor(&self) -> String {
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
            let mut SquareColor: Color = Color::Black;
            if (x + y) % 2 == 0 {
                SquareColor = Color::White
            }

            Self {
                square_color: SquareColor,
                piece,
                x,
                y,
            }
        }

        pub fn HasPiece(&self) -> Option<Piece> {
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

        fn add_piece(&mut self, piece: Piece) {
            self.piece = Option::Some(piece);
        }
    }

    pub struct ChessBoard {
        squares: [[Square; 8]; 8],
        remaining_pieces: Vec<Piece>,
    }

    impl ChessBoard {
        pub fn new() -> ChessBoard {
            let mut board: ChessBoard;
            let mut squares: [[Square; 8]; 8] = [[Square::new(None, 0, 0); 8]; 8];

            for x in 0..8 {
                for y in 0..8 {
                    squares[x][y] = Square::new(None, x as i8, y as i8);
                }
            }

            return ChessBoard {
                squares,
                remaining_pieces: vec![],
            };
        }

        pub fn HasPiece(&self, rank: i8, file: i8) -> Option<Piece> {
            return self.squares[rank as usize][file as usize].HasPiece();
        }

        pub fn FromFEN(fen: String) -> ChessBoard {
            let mut file = 0;
            let mut rank = 7;

            let mut board = ChessBoard::new();

            for x in fen.chars() {
                if x == '/' {
                    file = 0;
                    rank -= 1;
                } else {
                    if (x.is_numeric()) {
                        file += x.to_string().parse::<i8>().unwrap();
                    } else {
                        board.squares[rank as usize][file as usize]
                            .add_piece(Piece::new(x, rank as i8, file as i8));
                        file += 1;
                    }
                }
            }

            board
        }

        pub fn PrintBoard(&self) {
            for x in 0..8 {
                for y in 0..8 {
                    self.squares[x][y].print_square();
                }
                self.squares[x][0].print_square();
                println!()
            }
        }

        pub fn Move(&mut self, mv: String) -> Result<(), MoveStructureError> {
            if mv.len() < 3 {
                return Err(MoveStructureError);
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
            println!("\n{}, {} -> {}, {}", y_from, x_from, y_to, x_to);

            if let Some(piece) = self.HasPiece(x_from, y_from) {
                // normal move without any checking yet and no capture
                if self.HasPiece(x_to, y_to).is_none() {
                    if self.is_basic_valid_move(
                        piece.piece,
                        (y_from as i32, x_from as i32),
                        (y_to as i32, x_to as i32),
                        piece.color,
                    ) {
                        println!("is valid");
                        // add the piece to the designated square
                        self.squares[x_to as usize][y_to as usize].add_piece(piece);

                        // substract the piece from the previous square
                        self.squares[x_from as usize][y_from as usize].piece = None;
                    }
                }
            }

            return Ok(());
        }

        fn is_basic_valid_move(
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

    fn get_coords_from_letter(mv: char) -> Option<i8> {
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
