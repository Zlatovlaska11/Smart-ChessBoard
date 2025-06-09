use std::time::Duration;

use chessboard::{chess_game::ChessBoard, puzzles};
use tokio::{sync::mpsc, time::sleep};

pub mod chessboard;
pub mod config;
pub mod connector;
pub mod logger;
pub mod parser;

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = mpsc::channel(32);
//     let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
//
//     let puzzls = puzzles::load_puzzles("./puzzles.json");
//
//     println!("{:?}", puzzls);
//
//     let settings = config::Config::load("./config.toml".to_string());
//
//     let _ = tx.send("START\n".to_string()).await;
//
//     let thr = tokio::spawn(connector::run_server(rx, ready_tx, settings));
//
//     let startintg_pos = ready_rx.await.unwrap();
//
//     let mut chess = chessboard::chess_game::ChessBoard::from_fen(
//         startintg_pos.trim().to_string(),
//         tx,
//         chessboard::chess_game::TrainingMode::Game,
//         0,
//     );
//
//
//     ChessBoard::print_board(&chess);
//
//     let mv = chess.Move("E2E4".to_string()).await;
//
//     match mv {
//         Ok(_) => println!(),
//         Err(e) => println!("{}", e),
//     }
//
//     ChessBoard::print_board(&chess);
//
//     sleep(Duration::from_millis(1000)).await;
//     let mv = chess.Move("E7E5".to_string()).await;
//
//     match mv {
//         Ok(_) => println!(),
//         Err(e) => println!("{}", e),
//     }
//
//     ChessBoard::print_board(&chess);
//
//     let mv = chess.Move("G1F3".to_string()).await;
//
//     sleep(Duration::from_millis(100)).await;
//     match mv {
//         Ok(_) => println!(),
//         Err(e) => println!("{}", e),
//     }
//
//     ChessBoard::print_board(&chess);
//
//     chess.EndGame().await;
//     // update struct ChessBoard, add enum gameMode,
//
//     thr.await.unwrap();
// }

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();

    let puzzls = puzzles::load_puzzles("./puzzles.json");

    println!("{:?}", puzzls);

    let settings = config::Config::load("./config.toml".to_string());

    let _ = tx.send("START\n".to_string()).await;

    let thr = tokio::spawn(connector::run_server(rx, ready_tx, settings));

    // let startintg_pos = ready_rx.await.unwrap();

    let mut chess = chessboard::chess_game::ChessBoard::from_fen(
        puzzls.as_ref().unwrap()[0].fen.clone(),
        tx,
        chessboard::chess_game::TrainingMode::Puzzle {
            fen: puzzls.as_ref().unwrap()[0].fen.clone(),
            solution: puzzls.unwrap()[0].solution.clone(),
            current_step: 0,
        },
        0,
    );

    ChessBoard::print_board(&chess);

    let mv = chess.Move("G2G4".to_string()).await;

    match mv {
        Ok(_) => println!(),
        Err(e) => println!("{}", e),
    }

    ChessBoard::print_board(&chess);

    // sleep(Duration::from_millis(1000)).await;
    // let mv = chess.Move("E7E5".to_string()).await;
    //
    // match mv {
    //     Ok(_) => println!(),
    //     Err(e) => println!("{}", e),
    // }
    //
    // ChessBoard::print_board(&chess);
    //
    // let mv = chess.Move("G1F3".to_string()).await;
    //
    // sleep(Duration::from_millis(100)).await;
    // match mv {
    //     Ok(_) => println!(),
    //     Err(e) => println!("{}", e),
    // }
    //
    // ChessBoard::print_board(&chess);
    //
    chess.EndGame().await;
    // update struct ChessBoard, add enum gameMode,

    thr.await.unwrap();
}
