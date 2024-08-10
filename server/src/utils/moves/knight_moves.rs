use crate::utils::*;
const FILE_H_MASK: u64 =  0b1111111011111110111111101111111011111110111111101111111011111110;
const FILE_A_MASK: u64 = 0b0111111101111111011111110111111101111111011111110111111101111111;
pub fn generate_pseudo_moves(my_kings: u64, enemy: u64, occupied: u64, white: bool) -> u64 {
    let mut moves: u64 = 0;
    let my_pieces = !enemy & occupied;
    

    moves &= !my_pieces;
    return moves;
}