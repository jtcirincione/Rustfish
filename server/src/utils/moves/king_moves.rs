use crate::utils::*;
const FILE_H_MASK: u64 =  0b1111111011111110111111101111111011111110111111101111111011111110;
const FILE_A_MASK: u64 = 0b0111111101111111011111110111111101111111011111110111111101111111;
pub fn generate_pseudo_moves(my_kings: u64, enemy: u64, occupied: u64, white: bool) -> u64 {
    let mut moves: u64 = 0;
    let my_pieces = !enemy & occupied;
    moves |= (my_kings >> 1) & FILE_A_MASK; // move E
    moves |= (my_kings << 1) & FILE_H_MASK; // move W
    moves |= my_kings << 8; // move N
    moves |= my_kings >> 8; // move S
    moves |= (my_kings << 9) & FILE_H_MASK; // move NW
    moves |= (my_kings >> 7) & FILE_H_MASK; // move SW
    moves |= (my_kings << 7) & FILE_A_MASK; // move NE
    moves |= (my_kings >> 9) & FILE_A_MASK; // move SE

    moves &= !my_pieces;
    return moves;
}