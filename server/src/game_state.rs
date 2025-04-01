use crate::bitboard::Bitboard;
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
        return (board >> idx) & 1;
    }

    pub fn game_to_array(&self) -> Vec<String> {
        let mut board_vec: Vec<String> = Vec::new();

        for rank in 0..8 {
            for file in 0..8 {
                let i: u8 = rank * 8 + file;
                let mut found = false;
                for (name, bboard) in &self.board.pieces {
                    if Self::get_bit(bboard[0].board, i) == 1 {
                        board_vec.push(format!("w{}", name));
                        found = true;
                        break;
                    }
                    else if Self::get_bit(bboard[1].board, i) == 1 {
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

    pub fn get_valid_moves(&self, piece_type: &String, color: Turn) -> Vec<Move> {
        let moves: Vec<Move> = Vec::new();
        return moves;
    }

    pub fn make_move(&mut self, from: u64, to: u64, board_to_move_opt: Option<&mut Bitboard>, board_to_clear: Option<&mut Bitboard>) {
        let board_to_move_opt = self.board.get_proper_board(from);
        let board_to_clear = self.board.get_proper_board(to);
        //TODO: handle errors and early return
        let board_to_move = board_to_move_opt.unwrap();
        board_to_move.move_piece(from, to);
        if let Some(board) = board_to_clear {
            // do something if board isnt none
            board.clear_bit(to);
        }
        return ()

    }

    pub fn generate_pseudo_moves() {

    }
}
