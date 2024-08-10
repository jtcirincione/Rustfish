use crate::utils::moves::sliding_piece_moves;
pub fn generate_pseudo_moves(my_queens: u64, enemy: u64, occupied: u64, white: bool) -> u64 {
    let (idx, queen) = sliding_piece_moves::get_idx_of_lsb(my_queens);
    return sliding_piece_moves::h_v_moves(my_queens, occupied) | sliding_piece_moves::d_anti_moves(my_queens, occupied);
}