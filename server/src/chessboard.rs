use std::collections::HashMap;

use crate::enums::Turn;

const NOT_GH_MASK: u64 =
    0b0011111100111111001111110011111100111111001111110011111100111111;

const NOT_AB_MASK: u64 =
    0b1111110011111100111111001111110011111100111111001111110011111100;
const NOT_H_MASK: u64 =
    0b0111111101111111011111110111111101111111011111110111111101111111;
const NOT_G_MASK: u64 =
    0b1011111110111111101111111011111110111111101111111011111110111111;
const NOT_B_MASK: u64 =
    0b1111110111111101111111011111110111111101111111011111110111111101;
const NOT_A_MASK: u64 =
    0b1111111011111110111111101111111011111110111111101111111011111110;

const DOUBLE_PUSH_WP_MASK: u64 =
    0b0000000000000000000000000000000011111111000000000000000000000000;

const SINGLE_PUSH_WP_MASK: u64 =
    0b0000000011111111111111111111111111111111111111111111111111111111;

const DOUBLE_PUSH_BP_MASK: u64 =
    0b0000000000000000000000001111111100000000000000000000000000000000;

const SINGLE_PUSH_BP_MASK: u64 =
    0b1111111111111111111111111111111111111111111111111111111100000000;


#[derive(Clone)]
pub struct Chessboard {
    pub pieces: HashMap<String, [u64; 2]>, // HashMap where key is piece type and value is array for white/black
}

//TODO: FIGURE OUT HOW TO ADD KNIGHT ATTACKS TO SELF OBJECT
impl Chessboard {

    pub fn new() -> Chessboard {
        let mut pieces = HashMap::new();
        
        pieces.insert(String::from("queens"), [0x00000000000008, 0x1000000000000000]); // White on D1, Black on D8
        pieces.insert(String::from("kings"), [0x00000000000010, 0x0800000000000000]); // White on E1, Black on E8
        pieces.insert(String::from("rooks"), [0x00000000000081, 0x8100000000000000]); // White on A1 and H1; Black on A8 and H8
        pieces.insert(String::from("bishops"), [0x00000000000024, 0x2400000000000000]); // White on C1 and F1; Black on C8 and F8
        pieces.insert(String::from("knights"), [0x00000000000042, 0x4200000000000000]); // White on B1 and G1; Black on B8 and G8
        pieces.insert(String::from("pawns"), [0x000000000000FF00, 0x00FF000000000000]); // White on A2-H2; Black on A7-H7
        
        let board = Chessboard { pieces };

        board.precompute_knights();
        return board;
    }

    pub fn print(&self) {
        let mut occupancy: u64 = 0;
        for (_, val) in self.pieces.iter() {
            occupancy |= val[0];
            occupancy |= val[1];
        }

        for rank in (0..8).rev() {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", rank + 1);
                }
                let square: u8 = rank * 8 + file;
                print!("{} ", Chessboard::get_bit(occupancy, square))
            }
            println!();
        }
    }

    pub fn static_print(board: u64) {

        for rank in (0..8).rev() {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", rank + 1);
                }
                let square: u8 = rank * 8 + file;
                print!("{} ", Chessboard::get_bit(board, square))
            }
            println!();
        }
    }

    // static get_bit method
    pub fn get_bit(board: u64, idx: u8) -> u64 {
        return (board >> idx) & 1
    }

    pub fn get_occupancy(&self) -> u64 {
        let mut occupancy: u64 = 0;
        for (_, val) in self.pieces.iter() {
            occupancy |= val[0] | val[1];
        }
        return occupancy;
    }

    pub fn get_color_board(&self, color: Turn) -> u64 {
        let mut occupancy: u64 = 0;
        for (_, val) in self.pieces.iter() {
            occupancy |= if color == Turn::White { val[0] } else { val[1] };
        }
        return occupancy;
    }

    pub fn generate_knight_moves(&self, idx: usize) -> u64 {
        let mut knight_move: u64 = 0x0;
        let board: u64 = 1 << idx;
        let moves:[i32; 8] = [
            6,  // top left move restrict from G and H
            15,  // top left move restrict from H
            10,  // top right move, # restrict from A and B
            17,  // top right move, # restrict from A
            -6,  // bottom right move # restrict from A and B
            -15,  // bottom right move # restrict from A
            -10,  // bottom left move # restrict from G and H
            -17  // bottom left move # restrict from H
        ];
        for mov in moves {
            let mut position: u64 = 0;
            if mov > 0 {
                position = board << mov;
            }
            else {
                position = board >> -mov;
            }
            if mov == 6 || mov == -10 {
                position &= NOT_GH_MASK;
            }
            if mov == 15 || mov == -17 {
                position &= NOT_H_MASK;
            }
            if mov == 10 || mov == -6 {
                position &= NOT_AB_MASK;
            }
            if mov == -15 || mov == 17 {
                position &= NOT_A_MASK;
            }
            knight_move |= position;
        }
        return knight_move;

    }

    pub fn precompute_knights(&self) {
        let mut attacks: [u64; 64] = [0; 64];
        for i in 0..64 {
            attacks[i] = self.generate_knight_moves(i);
        }
    }

    pub fn precompute_kings(&self) {
        for i in 0..64 {
            self.KING_MOVES[i] = self.generate_king_moves(i);
        }
    }

    pub fn generate_king_moves(&self, idx: usize) -> u64 {
        let mut king_move = 0;
        let board: u64 = 1 << idx;

        let moves = [8, -8, 1, -1, 7, 9, -7, -9];
        
        for mov in moves {
            let mut potential_move: u64 = 0;
            if mov > 0 {
                potential_move = board << mov
            }
            else {
                potential_move = (board >> -mov);
            }

            if mov == -1 || mov == -9 || mov == 7{  // if piece is moving to the left
                potential_move &= NOT_H_MASK;
            }
            if mov == 1 || mov == 9 || mov == -7{ // if piece is moving to the right
                potential_move &= NOT_A_MASK;
            }

            king_move |= potential_move;
        }
        return king_move
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_print() {
        let board = Chessboard::new();
        board.print();
    }

    #[test]
    #[ignore]
    fn test_static_print() {
        Chessboard::static_print(0x00000000000010);
    }
}