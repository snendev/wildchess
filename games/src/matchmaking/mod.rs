use serde::{Deserialize, Serialize};

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{
    resource_exists, Event, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet,
};

use bevy_replicon::prelude::*;

pub mod components;

mod systems;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct MatchmakingSystems;

pub struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<RepliconCorePlugin>() {
            app.add_plugins((RepliconCorePlugin, ParentSyncPlugin));
        }
        app.add_client_event::<RequestJoinGameEvent>(ChannelKind::Ordered)
            .replicate::<components::GameRequestVariant>()
            .replicate::<components::GameRequestClock>()
            .replicate::<components::GameRequest>()
            .configure_sets(Update, MatchmakingSystems.run_if(has_authority))
            .add_systems(
                Update,
                (
                    systems::handle_game_requests,
                    systems::match_specified_game_requests,
                    systems::match_remaining_game_requests,
                    systems::assign_game_teams,
                    systems::spawn_game_entities,
                    systems::handle_visibility.run_if(resource_exists::<ConnectedClients>),
                    systems::despawn_empty_games,
                    systems::cleanup_game_entities,
                )
                    .chain()
                    .in_set(MatchmakingSystems),
            );
    }
}

#[derive(Clone)]
#[derive(Deserialize, Serialize)]
pub enum GameOpponent {
    Online,
    Local,
    AgainstBot,
    Analysis,
}

#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct RequestJoinGameEvent {
    // TODO: more configuration
    pub game: Option<components::GameRequestVariant>,
    pub clock: Option<components::GameRequestClock>,
    pub opponent: GameOpponent,
}
