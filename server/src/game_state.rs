use axum::Json;
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
    pub bitboards: std::collections::HashMap<String, (u64, String, String)>,
    
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
            bitboards: std::collections::HashMap::new(),
        }
    }

    pub fn to_json(self) -> Json<GameState> {
        Json(self)
    }

    pub fn get_bit(board: &u64, idx: u8) -> u64 {
        return (board >> (63 - idx)) & 1;
    }

    pub fn game_to_array(&self) -> Vec<String> {
        let mut board_vec: Vec<String> = Vec::new();
        
        for i in 0..64 {
            let mut found = false;
            for (key, val) in self.bitboards.into_iter() {
                if Self::get_bit(&val.0, i) == 1 {
                    found = true;
                    board_vec.push(format!("{}{}", val.1, val.2));
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

    pub fn make_move(&self, from: i32, to: i32, piece_type: &str) {
        let from = 63 - from;
        let to = 63 - to;
        return ()

    }
}
