use rand::Rng;

use chess::pieces::Behavior;

use super::PieceKind;

impl PieceKind {
    pub fn generate_king() -> Behavior {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            // classical king
            0 => Behavior::builder().radials().range(1).can_attack().build(),
            // weak king
            1 => Behavior::builder()
                .orthogonals()
                .range(1)
                .can_attack()
                .build(),
            // silver general king
            2 => Behavior::builder()
                .forward()
                .diagonals()
                .range(1)
                .can_attack()
                .build(),
            // gold general king
            _ => Behavior::builder()
                .forward()
                .sideways()
                .backward()
                .diagonal_forward()
                .range(1)
                .can_attack()
                .build(),
        }
    }
}
