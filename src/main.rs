use std::thread;

use chessboard::chess_game::{self, ChessBoard};
use tokio::sync::mpsc;

pub mod chessboard;
pub mod connector;
pub mod logger;
pub mod parser;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    let thr = tokio::spawn(connector::run_server(rx));

    // this is being created either too late or to early so the moves are never sent


    let mut chess = chessboard::chess_game::ChessBoard::FromFEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
        tx,
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
