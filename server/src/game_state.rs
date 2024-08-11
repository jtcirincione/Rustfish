use tower::util::error::optional::None;

use crate::utils::moves::*;
use crate::enums::Turn;
use crate::piece_move::Move;

#[derive(Clone)]
pub struct GameState {
    pub w_queens: u64,
    pub w_pawns: u64,
    pub w_rooks: u64,
    pub w_bishops: u64,
    pub w_king: u64,
    pub w_knights: u64,
    pub b_queens: u64,
    pub b_pawns: u64,
    pub b_rooks: u64,
    pub b_bishops: u64,
    pub b_king: u64,
    pub b_knights: u64,
    pub bitboards: std::collections::HashMap<String, u64>,
    pub history: Vec<u64>,
    pub white: bool,
    
}

impl GameState {
    pub fn new() -> GameState {
        let w_queens = 0x0000000000000008; let w_pawns = 0x000000000000FF00;
        let w_rooks = 0x0000000000000081; let w_bishops = 0x0000000000000024;
        let w_king = 0x0000000000000010; let w_knights = 0x0000000000000042;
        let b_queens = 0x0800000000000000; let b_pawns = 0x00FF000000000000;
        let b_rooks = 0x8100000000000000; let b_bishops = 0x2400000000000000;
        let b_king = 0x1000000000000000; let b_knights = 0x4200000000000000;

        return GameState {
            w_queens,
            w_pawns,
            w_rooks,
            w_bishops,
            w_king,
            w_knights,
            b_queens,
            b_pawns,
            b_rooks,
            b_bishops,
            b_king,
            b_knights,
            bitboards: std::collections::HashMap::from([
                ("wQ".to_string(), w_queens), ("wp".to_string(), w_pawns),
                ("wR".to_string(), w_rooks), ("wB".to_string(), w_bishops),
                ("wK".to_string(), w_king), ("wN".to_string(), w_knights),
                ("bQ".to_string(), b_queens), ("bp".to_string(), b_pawns),
                ("bR".to_string(), b_rooks), ("bB".to_string(), b_bishops),
                ("bK".to_string(), b_king), ("bN".to_string(), b_knights),
            ]),
            history: Vec::new(),
            white: true,
        }
    }

    pub fn get_bit(board: &u64, idx: u8) -> u64 {
        return (board >> (63 - idx)) & 1;
    }

    pub fn game_to_array(&self) -> Vec<String> {
        let mut board_vec: Vec<String> = Vec::new();
        
        for i in 0..64 {
            let mut found = false;
            for (key, val) in &self.bitboards {
                if Self::get_bit(&val, i) == 1 {
                    found = true;
                    board_vec.push(format!("{}", key));
                    break;
                }
            }
            if !found {
                board_vec.push(String::from(""));
            }
        }
        
        return board_vec;
    }

    pub fn actually_move(&mut self, piece_type: &String, from: u64, to: u64, capture_type: &Option<String>) {
        println!("yay");
        let mask = 1 << (63 - from);
        if let Some(board) = self.bitboards.get_mut(piece_type) {
            *board &= !mask; // Remove piece from `from`
            *board |= 1 << (63 - to); // Add piece to `to`
            println!("found {}", board);
            if let Some(capture_string) = capture_type {
                if let Some(capture_board) = self.bitboards.get_mut(capture_string) {
                    let to_mask = 1 << (63 - to);
                    *capture_board &= !to_mask;
                    println!("this was a capture move.");
                }
            }
        }
        
        return ();
    }

    pub fn get_valid_moves(&self, piece_type: &String, color: Turn) -> Vec<Move> {
        let mut moves = Vec::new();
        if let Some(board) = self.bitboards.get_mut(piece_type) {
            match piece_type {
                "wp" => pawn_moves::generate_pseudo_moves(board, enemy, occupied, white)
            }
        }
        else {
            return Vec::new();
        }
    }

    pub fn make_move(&mut self, from: u64, to: u64, piece_type: &String, capture_type: &Option<String>) {
        //TODO: handle errors and early return
        self.actually_move(piece_type, from, to, capture_type);
        
        return ()

    }

    pub fn generate_pseudo_moves() {

    }
}
