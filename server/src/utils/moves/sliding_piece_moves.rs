use crate::utils::*;
const FILE_H_MASK: u64 =  0b1111111011111110111111101111111011111110111111101111111011111110;
const FILE_A_MASK: u64 = 0b0111111101111111011111110111111101111111011111110111111101111111;


// from top right to bottom left
const diagonal_masks: [u64; 15] = [
    0x8000000000000000, 0x4080000000000000, 0x2040800000000000, 0x1020408000000000, 
    0x810204080000000, 0x408102040800000, 0x204081020408000, 0x102040810204080, 
    0x1020408102040, 0x10204081020, 0x102040810, 0x1020408,
    0x10204, 0x102, 0x1
];
// from top left to bottom right
const antidiagonal_masks: [u64; 15] = [ 0x100000000000000, 0x201000000000000, 0x402010000000000, 0x804020100000000,
                0x1008040201000000, 0x2010080402010000, 0x4020100804020100, 0x8040201008040201,
                0x80402010080402, 0x804020100804, 0x8040201008, 0x80402010, 0x804020, 
                0x8040, 0x80
];

// from rank 1 to 8
const rank_masks: [u64; 8] = [0xFF00000000000000, 0xFF000000000000, 0xFF0000000000, 0xFF00000000,
            0xFF000000, 0xFF0000, 0xFF00, 0xFF];

// from file H to A
const file_masks: [u64; 8] = [
    0x8080808080808080,
    0x4040404040404040,
    0x2020202020202020,
    0x1010101010101010,
    0x808080808080808,
    0x404040404040404,
    0x202020202020202,
    0x101010101010101
];


const index64: [u64; 64] = [
    0, 47,  1, 56, 48, 27,  2, 60,
   57, 49, 41, 37, 28, 16,  3, 61,
   54, 58, 35, 52, 50, 42, 21, 44,
   38, 32, 29, 23, 17, 11,  4, 62,
   46, 55, 26, 59, 40, 36, 15, 53,
   34, 51, 20, 43, 31, 22, 10, 45,
   25, 39, 14, 33, 19, 30,  9, 24,
   13, 18,  8, 12,  7,  6,  5, 63
];


fn reverse_bits(mut b: u64) -> u64{
        b = (b & 0x5555555555555555) << 1 | ((b >> 1) & 0x5555555555555555);
        b = (b & 0x3333333333333333) << 2 | ((b >> 2) & 0x3333333333333333);
        b = (b & 0x0f0f0f0f0f0f0f0f) << 4 | ((b >> 4) & 0x0f0f0f0f0f0f0f0f);
        b = (b & 0x00ff00ff00ff00ff) << 8 | ((b >> 8) & 0x00ff00ff00ff00ff);

        return (b << 48) | ((b & 0xffff0000) << 16) | ((b >> 16) & 0xffff0000) | (b >> 48);
}

//returns a tuple of the idx of LSB and the board with that bit 0'ed out
pub fn get_idx_of_lsb(board: u64) -> (usize, u64) {
    if (board == 0) {
        panic!("Board is empty! couldn't get lsb");
    }
    let idx = ((board ^ (board - 1)) * 0x03f79d71b4cb0a89 >> 58) as usize;
    let mut i = index64[idx];
    
    let shift = i;
    i = 63 - i;
    return (i as usize, (board & !(1 << shift)));
}

pub fn h_v_moves(my_sliding_piece: u64, occupied: u64) -> u64 {
        let s = my_sliding_piece;
        let (idx, _) = get_idx_of_lsb(my_sliding_piece);
        
        let occ_h = occupied & rank_masks[idx/8];
        let occ_v = occupied & file_masks[idx%8];
        
        let horizontal = (occ_h - (2 * s)) ^ reverse_bits(reverse_bits(occ_h) - 2 * reverse_bits(s));
        let vertical = (occ_v - (2 * s)) ^ reverse_bits(reverse_bits(occ_v) - 2 * reverse_bits(s));
        return (horizontal & rank_masks[idx/8]) | (vertical & file_masks[idx%8]);
}

pub fn d_anti_moves(my_sliding_piece: u64, occupied: u64) -> u64 {
        let s = my_sliding_piece;
        let (idx, _) = get_idx_of_lsb(my_sliding_piece);
        let occ_d = occupied & diagonal_masks[(idx/8) + (idx%8)];
        let occ_a = occupied & antidiagonal_masks[(idx/8) + 7 - (idx%8)];
        
        let diagonal = (occ_d - (2 * s)) ^ reverse_bits(reverse_bits(occ_d) - 2 * reverse_bits(s)); 
        let antidiagonal = (occ_a - (2 * s)) ^ reverse_bits(reverse_bits(occ_a) - 2 * reverse_bits(s));
        return (diagonal & diagonal_masks[(idx/8) + (idx%8)]) | (antidiagonal & antidiagonal_masks[(idx/8) + 7 - (idx%8)]);

}