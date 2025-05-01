use chessboard::chess_game::{self, ChessBoard};

pub mod chessboard;
pub mod logger;
pub mod parser;

fn main() {
    let mut chess = chessboard::chess_game::ChessBoard::FromFEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
    );

    ChessBoard::PrintBoard(&chess);

    let mv = chess.Move("E2E4".to_string());

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::PrintBoard(&chess);

    let mv = chess.Move("E7E5".to_string());

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::PrintBoard(&chess);

    let mv = chess.Move("G1F3".to_string());

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::PrintBoard(&chess);
}
