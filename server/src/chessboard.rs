use std::collections::HashMap;

#[derive(Clone)]
pub struct Chessboard {
    pub pieces: HashMap<String, [u64; 2]>, // HashMap where key is piece type and value is array for white/black
}

impl Default for Chessboard {
    fn default() -> Self {
        let mut pieces = HashMap::new();
        
        pieces.insert(String::from("queens"), [0x00000000000010, 0x1000000000000000]);
        pieces.insert(String::from("rooks"), [0x00000000000081, 0x8100000000000000]);
        pieces.insert(String::from("kings"), [0x00000000000008, 0x0800000000000000]);
        pieces.insert(String::from("bishops"), [0x00000000000024, 0x2400000000000000]);
        pieces.insert(String::from("knights"), [0x00000000000042, 0x4200000000000000]);
        pieces.insert(String::from("pawns"), [0x000000000000FF00, 0x00FF000000000000]);

        Chessboard { pieces }
    }
}

impl Chessboard {
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
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_print() {
        let board = Chessboard::default();
        board.print();
    }

    #[test]
    #[ignore]
    fn test_static_print() {
        Chessboard::static_print(0x00000000000010);
    }
}