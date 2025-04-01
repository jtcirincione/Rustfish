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

pub struct AttackTables {
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],
    bishop: [u64; 64],
    rook: [u64; 64],
    queen: [u64; 64],
}


impl AttackTables {

    //statics
    fn get_bit(board: u64, idx: usize) -> u64 {
        return (board >> idx) & 1
    }

    fn print(board: u64) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", rank + 1);
                }
                let square: usize = rank * 8 + file;
                print!("{} ", AttackTables::get_bit(board, square))
            }
            println!();
        }
    }

    pub fn generate() -> AttackTables {
        AttackTables {
            pawn: AttackTables::precompute_pawns(),
            knight: AttackTables::precompute_knights(),
            king: AttackTables::precompute_kings(),
            bishop: [0; 64],
            rook: [0; 64],
            queen: [0; 64],
        }
    }

    fn generate_king_moves(idx: usize) -> u64 {
        let mut king_move = 0;
        let board: u64 = 1 << idx;
        let moves = [8, -8, 1, -1, 7, 9, -7, -9];
        
        for mov in moves {
            let mut potential_move: u64;
            if mov > 0 {
                potential_move = board << mov
            }
            else {
                potential_move = board >> -mov;
            }

            if mov == -1 || mov == -9 || mov == 7 {  // if piece is moving to the left
                potential_move &= NOT_H_MASK;
            }
            if mov == 1 || mov == 9 || mov == -7 { // if piece is moving to the right
                potential_move &= NOT_A_MASK;
            }

            king_move |= potential_move;
        }
        return king_move
    }

    pub fn precompute_kings() -> [u64; 64]{
        let mut king_moves: [u64; 64] = [0; 64];
        for i in 0..64 {
            king_moves[i] = AttackTables::generate_king_moves(i);
        }
        king_moves
    }

    fn generate_knight_moves(idx: usize) -> u64 {
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
            let mut position: u64;
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

    fn precompute_knights() -> [u64; 64] {
        let mut attacks: [u64; 64] = [0; 64];
        for i in 0..64 {
            attacks[i] = AttackTables::generate_knight_moves(i);
        }
        return attacks;
    }

    fn precompute_pawns() -> [[u64; 64]; 2] {
        let mut pawns: [[u64; 64]; 2] = [[0; 64]; 2];
        for square in 0..64 {
            let mut wattack_squares: u64 = 0;
            let mut battack_squares: u64 = 0;
            let pawn = 1 << square;
            wattack_squares |= (pawn << 9) & NOT_A_MASK; // right attack
            wattack_squares |= (pawn << 7) & NOT_H_MASK; // left attack

            battack_squares |= (pawn >> 7) & NOT_A_MASK; // right attack
            battack_squares |= (pawn >> 9) & NOT_H_MASK; // left attack

            pawns[0][square] = wattack_squares;
            pawns[1][square] = battack_squares;
        }
        return pawns;
    }
}