pub struct Move {
    to: u64,
    from: u64,
    mtype: u8 // 0: quiet, 1: castle
}

impl Move {
    pub fn new(to: u64, from: u64, mtype: u8) -> Move {
        return Move {
            to,
            from,
            mtype
        };
    }
}