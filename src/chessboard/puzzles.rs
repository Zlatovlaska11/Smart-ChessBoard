use std::error::Error;
use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Puzzle {
    pub fen: String,
    pub solution: Vec<String>, // UCI format e.g., "f4b8"
}

pub fn load_puzzles(filename: &str) -> Result<Vec<Puzzle>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let puzzles: Vec<Puzzle> = serde_json::from_reader(reader)?;
    Ok(puzzles)
}
