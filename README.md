# Smart-ChessBoard Documentation

**Last Updated**: 2025-06-04

## Table of Contents

1. [Project Overview](#project-overview)
2. [Project Architecture](#project-architecture)
3. [Client Architecture](#client-architecture)

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


