
#[derive(Serialize, Deserialize)]
pub struct AttackTables {
    pawn: [[u64; 64]; 2],
    knight: [u64; 64],
    king: [u64; 64],
    bishop: [u64; 64],
    rook: [u64; 64],
    queen: [u64; 64],
}
