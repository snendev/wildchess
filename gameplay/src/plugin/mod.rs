use bevy::prelude::{App, Commands, IntoSystemConfigs, Plugin, Startup, Update};

use chess::team::Team;

use crate::{
    components::{PlayerBundle, Turn},
    IssueMoveEvent, IssuePromotionEvent, RequestPromotionEvent, TurnEvent,
};

mod capture;
mod targets;
mod turns;

fn initialize_players(mut commands: Commands) {
    commands.spawn((PlayerBundle::new(Team::White), Turn));
    commands.spawn(PlayerBundle::new(Team::Black));
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnEvent>()
            .add_event::<IssueMoveEvent>()
            .add_event::<IssuePromotionEvent>()
            .add_event::<RequestPromotionEvent>()
            .add_systems(Startup, initialize_players)
            .add_systems(
                Update,
                (
                    turns::detect_turn,
                    turns::execute_turn,
                    capture::capture_pieces,
                    (targets::calculate_targets, turns::end_turn),
                )
                    .chain(),
            );
    }
}
