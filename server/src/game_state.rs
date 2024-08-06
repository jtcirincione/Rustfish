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
    
}

impl GameState {
    pub fn new() -> GameState {
        let w_queens = 0x0000000000000008;
        let w_pawns = 0x000000000000FF00;
        let w_rooks = 0x0000000000000081;
        let w_bishops = 0x0000000000000024;
        let w_king = 0x0000000000000010;
        let w_knights = 0x0000000000000042;
        let b_queens = 0x0800000000000000;
        let b_pawns = 0x00FF000000000000;
        let b_rooks = 0x8100000000000000;
        let b_bishops = 0x2400000000000000;
        let b_king = 0x1000000000000000;
        let b_knights = 0x4200000000000000;
        

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
                ("wQ".to_string(), w_queens),
                ("wp".to_string(), w_pawns),
                ("wR".to_string(), w_rooks),
                ("wB".to_string(), w_bishops),
                ("wK".to_string(), w_king),
                ("wN".to_string(), w_knights),
                ("bQ".to_string(), b_queens),
                ("bp".to_string(), b_pawns),
                ("bR".to_string(), b_rooks),
                ("bB".to_string(), b_bishops),
                ("bK".to_string(), b_king),
                ("bN".to_string(), b_knights),

            ]),
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
        
        //the plan is to append a 'color'[piecetype] to the vector for whatever type of piece has a 1 at that index. how do I do this
        
        return board_vec;
    }

    pub fn actually_move(board: &mut u64, from: u64, to: u64) {
        let mask = 1 << (63 - from);
        if *board & mask != 0 {
            *board &= !mask; // Remove piece from `from`
            *board |= 1 << (63 - to); // Add piece to `to`
        }
        return ();
    }

    pub fn make_move(&mut self, from: u64, to: u64, piece_type: &String) {
        //TODO: handle errors and early return
        if let Some(board) = self.bitboards.get_mut(piece_type) {
            Self::actually_move(board, from, to);
        }
        
        return ()

    }
}
