use chess::pieces::{Behavior, Pattern};

pub fn pawn() -> Behavior {
    Behavior::default()
        .with_pattern(Pattern::forward().range(1).cannot_attack())
        .with_pattern(Pattern::diagonal_forward().range(1).can_attack())
}

// no castling
pub fn king() -> Behavior {
    Behavior::builder().radials().range(1).can_attack().build()
}

pub fn knight() -> Behavior {
    Behavior::builder()
        .knight_jumps()
        .range(1)
        .can_attack()
        .build()
}

pub fn bishop() -> Behavior {
    Behavior::builder().diagonals().can_attack().build()
}

pub fn rook() -> Behavior {
    Behavior::builder().orthogonals().can_attack().build()
}

pub fn queen() -> Behavior {
    Behavior::builder().radials().can_attack().build()
}
