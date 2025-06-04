use std::time::Duration;

use chessboard::chess_game::ChessBoard;
use tokio::{sync::mpsc, time::sleep};

pub mod chessboard;
pub mod connector;
pub mod logger;
pub mod parser;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();

    let _ = tx.send("START\n".to_string()).await;

    let _thr = tokio::spawn(connector::run_server(rx, ready_tx));

    let startintg_pos = ready_rx.await.unwrap();

    let mut chess =
        chessboard::chess_game::ChessBoard::from_fen(startintg_pos.trim().to_string(), tx);

    ChessBoard::print_board(&chess);

    let mv = chess.Move("E2E4".to_string()).await;

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::print_board(&chess);

    sleep(Duration::from_secs(5)).await;
    let mv = chess.Move("E7E5".to_string()).await;

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::print_board(&chess);

    let mv = chess.Move("G1F3".to_string()).await;

    sleep(Duration::from_millis(100)).await;
    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::print_board(&chess);
}
