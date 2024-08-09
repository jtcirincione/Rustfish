const BLACK_2MOVE_ROW: u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
const WHITE_2MOVE_ROW: u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
const BLACK_PASSANT_ROW: u64 = 0b0000000000000000111111110000000000000000000000000000000000000000;
const WHITE_PASSANT_ROW: u64 = 0b0000000000000000000000000000000000000000111111110000000000000000;
const FILE_H_MASK: u64 =  0b1111111011111110111111101111111011111110111111101111111011111110;
const FILE_A_MASK: u64 = 0b0111111101111111011111110111111101111111011111110111111101111111;
const WHITE_PROMOTE_MASK: u64 = 0xFF00000000000000;
const BLACK_PROMOTE_MASK: u64 = 0xFF;

pub fn generate_pseudo_moves(my_pawns: &u64, enemy: &u64, occupied: &u64, white: bool) -> u64 {
    let push;
    let double_push;
    // skip en passant for now
    // let en_passant;
    let promote;
    let mut left_attacks;
    let mut right_attacks;
    if white {
        double_push = (my_pawns << 16) & !occupied;
        push = (my_pawns << 8) & !occupied;
        left_attacks = my_pawns << 9 & FILE_H_MASK;
        right_attacks = my_pawns << 7 & FILE_A_MASK;
        promote = (left_attacks | right_attacks) & WHITE_PROMOTE_MASK;
        left_attacks &= !WHITE_PROMOTE_MASK;
        right_attacks &= !WHITE_PROMOTE_MASK;
        
    } 
    else {
        double_push = (my_pawns >> 16) & !occupied;
        push = (my_pawns >> 8) & !occupied;
        left_attacks = my_pawns >> 7 & FILE_H_MASK;
        right_attacks = my_pawns >> 9 & FILE_A_MASK;
        promote = (left_attacks | right_attacks) & BLACK_PROMOTE_MASK;
        left_attacks &= !BLACK_PROMOTE_MASK;
        right_attacks &= !BLACK_PROMOTE_MASK;
    }

    return push | double_push | left_attacks | right_attacks | promote;
}