use bevy::prelude::Bundle;

mod clock;
pub use clock::Clock;

mod player;
pub use player::Player;

mod turn;
pub use turn::Turn;

use chess::team::Team;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    team: Team,
    clock: Clock,
}

impl PlayerBundle {
    pub fn new(team: Team) -> Self {
        PlayerBundle {
            player: Player,
            team,
            clock: Clock::default(),
        }
    }
}
