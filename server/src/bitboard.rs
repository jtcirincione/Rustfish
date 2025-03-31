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
}