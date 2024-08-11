use crate::utils::moves::*;
pub fn generate_pseudo_moves(my_rooks: u64, enemy: u64, occupied: u64, white: bool) -> u64 {
    let my_pieces = !enemy & occupied;
    return sliding_piece_moves::h_v_moves(my_rooks, occupied) & !my_pieces;
}