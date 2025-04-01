#[derive(Clone)]
pub struct Bitboard {
    pub board: u64,
}
impl Bitboard {
    pub fn new(b: u64) -> Bitboard {
        return Bitboard {
            board: b
        }
    }

    pub fn bit_scan_forward(bitboard: u64) -> u32 {
        return bitboard.trailing_zeros();  //TODO: is traling 0s efficient
    }

    pub fn get_bit(&self, idx: u64) -> u64 {
        return (self.board >> idx) & 1
    }

    pub fn set_bit(&mut self, idx: u64) {
        self.board |= 1 << idx
    }

    pub fn clear_bit(&mut self, idx: u64) {
        self.board &= !(1 << idx)
    }

    pub fn move_piece(&mut self, from:u64, to:u64) {
        self.set_bit(to);
        self.clear_bit(from);
    }
}