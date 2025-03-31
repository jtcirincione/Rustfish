use crate::chessboard::Chessboard;
use crate::enums::Turn;
use crate::piece_move::Move;

#[derive(Clone)]
pub struct GameState {
    pub board: Chessboard,
    pub history: Vec<u64>,
    pub white: bool,
    
}

impl GameState {
    pub fn new() -> GameState {
        let board = Chessboard::new();

        return GameState {
            board,
            history: Vec::new(),
            white: true,
        }
    }

    pub fn get_bit(board: u64, idx: u8) -> u64 {
        return (board >> (63 - idx)) & 1;
    }

    pub fn game_to_array(&self) -> Vec<String> {
        let mut board_vec: Vec<String> = Vec::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let i: u8 = rank * 8 + file;
                let mut found = false;
                for (name, piece) in &self.board.pieces {
                    if Self::get_bit(piece[0], i) == 1 {
                        board_vec.push(format!("w{}", name));
                        found = true;
                        break;
                    }
                    else if Self::get_bit(piece[1], i) == 1 {
                        board_vec.push(format!("b{}", name));
                        found = true;
                        break;
                    }
                }
                if !found {
                    board_vec.push(String::from(""))
                }
            }
        }
        return board_vec;
    }

    pub fn actually_move(&mut self, piece_type: &String, from: u64, to: u64, capture_type: &Option<String>) {
        println!("yay");
        let mask = 1 << (63 - from);
        
        return ();
    }

    pub fn get_valid_moves(&self, piece_type: &String, color: Turn) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        return moves;
    }

    pub fn make_move(&mut self, from: u64, to: u64, piece_type: &String, capture_type: &Option<String>) {
        //TODO: handle errors and early return
        self.actually_move(piece_type, from, to, capture_type);
        
        return ()

    }

    pub fn generate_pseudo_moves() {

    }
}
