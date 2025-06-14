# Smart-ChessBoard Documentation

**Last Updated**: 2025-06-04

## Table of Contents

1. [Project Overview](#project-overview)
2. [Project Architecture](#project-architecture)
3. [Client Architecture](#client-architecture)
4. [System Components](#system-components)
   - [Chessboard Module](#chessboard-module)
   - [Connector Module](#connector-module)
   - [Communication Protocol](#communication-protocol)
5. [Data Structures](#data-structures)
   - [ChessBoard](#chessboard)
   - [Piece](#piece)
   - [Square](#square)
   - [Color](#color)
   - [PieceType](#piecetype)
6. [Core Functionalities](#core-functionalities)
   - [Board Initialization](#board-initialization)
   - [Move Validation](#move-validation)
   - [Communication with Server](#communication-with-server)
7. [Implementation Examples](#implementation-examples)
8. [Future Development](#future-development)

## Project Overview

Smart-ChessBoard is a Rust-based embedded client for a smart chess board system. The project aims to create a physical chess board that:

- Detects and validates moves in real-time
- Communicates with a backend server for move processing
- Displays the current game state via a web interface
- Can potentially integrate with chess engines or online play

The client is written in Rust for safety, speed, and low-level control, while the backend server is implemented in Go for simplicity, performance, and concurrency.

## Project Architecture

The system follows a three-tier architecture:

```
+----------------------------+       +---------------------------+       +-------------------------------------+
|     Rust Client (Board)    | --->  |      Go Server (Backend)  | --->  |   Frontend (Svelte)                 |
| - Runs on RPi / Arduino    |       | - forwards moves to fe    |       | - ui of the app                     |
| - Reads sensor input       |       | - holds connection via ws |       | - ws conn with go server            |
| - Sends moves via tcp      |       |                           |  <--- | - svelte for simplicity and to learn|
+----------------------------+       +---------------------------+       +-------------------------------------+
```

The Rust client is designed to run on embedded hardware like Raspberry Pi or Arduino, where it will:
1. Initialize the board state (currently from FEN notation)
2. Process user moves on the physical board
3. Validate basic move legality
4. Send move information to the server
5. Update the local board state 


## Client Architecture

The system is multithreaded with a channel for chessboard to sender communication

```
+----------------------------------+       +---------------------------+       
|         ChessBoard               | --->  |      Connector Module     |
| - Creates a initial state        |       | - forwards moves to fe    |      
| - move are being sent to channel |       | - holds connection via ws |     
+----------------------------------+       +---------------------------+
```

The client architecture:
1. Event based -> channel awaits message that is sent on every move
2. Move is sent to the [Sender](#connector-module) and send to backend
3. On start of each game server is initialized and waits for a start position (fen)
4. On game end "END" signal is sent to end the game and trigger frontend functionality


## System Components

### Chessboard Module

The core of the system, implemented in `src/chessboard/mod.rs`, contains the chess logic:

- Board representation and state management
- Piece movement rules and validation
- FEN notation parsing for board setup
- Console visualization of the chess board

### Errors

Custom error are implemented in `src/chessboard/errors.rs` that contains all the custom errors

- Custom errors
- Extendable with custom errors
- `InvalidMove` and `InvalidMoveStructure` errors
- Used in [ChessBoard](#chessboard-module) fen parsing and move making

### Config

Config loading module to load config from toml files (default values implemented)

- Config struct
- Extendable for future model implementation 
- Serde parsing 
- Used in main module to init the connector module

### Logger 

Custom logger implementation in `src/logger.rs`

- Logger implementation with info and error mode

### Connector Module

Handles communication with the backend server:

- Establishes TCP connection to the server
- Sends moves to the server
- Receives responses and initial board position
- Manages asynchronous communication using Tokio

### Communication Protocol

The client and server communicate over a simple TCP-based protocol:

1. Client connects to server (default: 127.0.0.1:3333)
2. Client sends "START" to initialize a game
3. Server responds with initial board position in FEN notation
4. Client sends moves in simple algebraic notation (e.g., "E2E4")
5. Server validates moves and updates game state

## Data Structures

### ChessBoard

Main class representing the chess board:

```rust
pub struct ChessBoard {
    squares: [[Square; 8]; 8],
    remaining_pieces: Vec<Piece>,
    sender: mpsc::Sender<String>,
}
```

- `squares`: 8x8 grid of Square objects representing the board
- `remaining_pieces`: List of pieces currently on the board
- `sender`: Channel for sending moves to the server

### Piece

Represents a chess piece:

```rust
pub struct Piece {
    color: Color,
    piece: PieceType,
    pub x: i8,
    pub y: i8,
}
```

- `color`: Color of the piece (White/Black)
- `piece`: Type of the piece (Pawn, Rook, etc.)
- `x`, `y`: Position coordinates on the board

### Square

Represents a square on the chess board:

```rust
struct Square {
    square_color: Color,
    piece: Option<Piece>,
    x: i8,
    y: i8,
}
```

- `square_color`: Color of the square
- `piece`: Optional piece occupying the square
- `x`, `y`: Position coordinates

### Color

Enum representing colors in chess:

```rust
enum Color {
    White,
    Black,
}
```

### PieceType

Enum representing types of chess pieces:

```rust
enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
    King,
    Queen,
}
```

## Core Functionalities

### Board Initialization

The board can be initialized in two ways:

1. Empty board:
   ```rust
   let mut chess = ChessBoard::new(sender);
   ```

2. From FEN notation:
   ```rust
   let mut chess = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(), sender);
   ```

FEN (Forsyth–Edwards Notation) is a standard notation for describing chess positions, where:
- Lowercase letters represent black pieces
- Uppercase letters represent white pieces
- Numbers represent empty squares
- Forward slashes separate ranks

### Move Validation

Moves are validated in a two-step process:

1. Basic structure validation (correct notation format)
2. Chess rule validation based on piece type:
   - Pawns: Forward movement and diagonal captures
   - Knights: L-shaped movement
   - Bishops: Diagonal movement
   - Rooks: Horizontal and vertical movement
   - Queens: Combination of bishop and rook movement
   - Kings: One square in any direction

Example usage:
```rust
let move_result = chess.Move("E2E4".to_string()).await;
match move_result {
    Ok(_) => println!("Move successful"),
    Err(e) => println!("Error: {}", e),
}
```

### Communication with Server

The client communicates with the server using a TCP connection:

1. Establish connection:
   ```rust
   let connection = tcp_connection::client::new("127.0.0.1".to_string(), 3333).await;
   ```

2. Start game and receive initial position:
   ```rust
   let position = connection.GameStart().await;
   ```

3. Send moves to server:
   ```rust
   connection.send(chess_move.as_bytes()).await;
   ```

## Implementation Examples

### Creating a Chess Board and Making Moves

```rust
#[tokio::main]
async fn main() {
    // Setup communication channels
    let (tx, rx) = mpsc::channel(32);
    let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();

    // Initialize connection with server
    let _ = tx.send("START\n".to_string()).await;
    let _thr = tokio::spawn(connector::run_server(rx, ready_tx));

    // Get starting position from server
    let starting_pos = ready_rx.await.unwrap();
    
    // Initialize chess board with FEN notation
    let mut chess = chessboard::chess_game::ChessBoard::from_fen(starting_pos.trim().to_string(), tx);
    
    // Print the board
    ChessBoard::print_board(&chess);

    // Make a move
    let mv = chess.Move("E2E4".to_string()).await;
    match mv {
        Ok(_) => println!("Move successful"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Print updated board
    ChessBoard::print_board(&chess);
}
```

## Future Development

Planned features for upcoming development:

- [ ] Hardware integration with real chess board sensors
- [ ] Support for UCI chess engines (e.g., Stockfish)
- [ ] Multiplayer/remote play capabilities
- [ ] Puzzle/training mode with guided LEDs
- [ ] Enhanced move validation (check, checkmate, castling, en passant)
- [ ] Complete web-based UI for game visualization
- [ ] Game history and analysis tools

Current implementation limitations:
- Basic move validation only (no advanced chess rules)
- No detection of check or checkmate
- Limited piece capture functionality
- No support for special moves (castling, en passant)
- Simulated board input rather than physical sensors
