use crate::utils::moves::sliding_piece_moves;
pub fn generate_pseudo_moves(my_bishops: u64, enemy: u64, occupied: u64, white: bool) -> u64 {
    let my_pieces = !enemy & occupied;
    return sliding_piece_moves::d_anti_moves(my_bishops, occupied) & !my_pieces;
}