use rand::Rng;

use crate::components::{Behavior, PieceConfiguration, PieceKind};

impl PieceKind {
    pub fn generate_king() -> PieceConfiguration {
        let mut rng = rand::thread_rng();
        PieceConfiguration {
            kind: PieceKind::King,
            behavior: match rng.gen_range(0..=2) {
                0 => Behavior::builder().radials().range(1).can_attack().build(),
                1 => Behavior::builder()
                    .orthogonals()
                    .range(1)
                    .can_attack()
                    .build(),
                _ => Behavior::builder()
                    .forward()
                    .diagonal_backward()
                    .range(1)
                    .can_attack()
                    .build(),
            },
            promotable: None,
        }
    }
}
