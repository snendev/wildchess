use bevy::prelude::Component;

use crate::components::Team;

mod clock;

mod turn;
pub use turn::Turn;

#[derive(Component)]
pub struct Player;

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
            team: Team,
            clock: Clock::default(),
        }
    }
}
