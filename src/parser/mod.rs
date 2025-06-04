use std::usize;

use crate::{chessboard::chess_game::ChessBoard, logger::Logger};

pub struct piece_map {
    map: [[bool; 8]; 8],
    chessboard: ChessBoard,
}

pub trait map_parser {
    fn parse(map: piece_map) -> String;
    fn get_diff(map: &piece_map) -> Vec<(u8, u8)>;
}

impl map_parser for piece_map {
    fn parse(map: piece_map) -> String {
        let diff = Self::get_diff(&map);

        let mv = map
            .chessboard
            .has_piece(diff[0].0 as i8, diff[0].1 as i8)
            .or(map.chessboard.has_piece(diff[1].0 as i8, diff[1].1 as i8));

        if mv.is_none() {
            Logger.e("eror ocured while parsing the move");
            panic!();
        }

        let rank = mv.unwrap().x;
        let file = mv.unwrap().y;

        if diff[0].0 == rank as u8 && diff[0].1 == file as u8 {
            return format!("{}{}{}{}", rank, file, diff[1].0, diff[1].1);
        }

        // TODO add the letter name convention for the move parsing
        return format!("{}{}{}{}", rank, file, diff[0].0, diff[0].1);
    }

    fn get_diff(map: &piece_map) -> Vec<(u8, u8)> {
        let mut moves = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                if (map.chessboard.has_piece(x, y).is_some() && !map.map[x as usize][y as usize])
                    || (map.map[x as usize][y as usize] && map.chessboard.has_piece(x, y).is_none())
                {
                    moves.push((x as u8, y as u8));
                }
            }
        }

        return moves;
    }
}
