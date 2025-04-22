use chessboard::chess_game::{self, ChessBoard};

pub mod chessboard;

fn main() {
    let mut chess = chessboard::chess_game::ChessBoard::FromFEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
    );

    ChessBoard::PrintBoard(&chess);


    chess.Move("E2E4".to_string());

    println!();

    ChessBoard::PrintBoard(&chess);

    println!();

    chess.Move("E7E5".to_string());

    println!();

    ChessBoard::PrintBoard(&chess);

}
