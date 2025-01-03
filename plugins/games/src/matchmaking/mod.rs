use serde::{Deserialize, Serialize};

use bevy::{
    ecs::entity::MapEntities,
    prelude::{
        resource_exists, App, Entity, EntityMapper, Event, IntoSystemConfigs, IntoSystemSetConfigs,
        Plugin, SystemSet, Update,
    },
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
        app.add_client_event::<RequestJoinGameEvent>(ChannelKind::Ordered)
            .add_client_event::<LeaveGameEvent>(ChannelKind::Ordered)
            .replicate::<components::GameRequestVariant>()
            .replicate::<components::GameRequestClock>()
            .replicate::<components::GameRequest>()
            .configure_sets(Update, MatchmakingSystems.run_if(has_authority))
            .add_systems(
                Update,
                (
                    systems::handle_game_requests,
                    systems::handle_leave_events,
                    systems::match_game_requests,
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

#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct LeaveGameEvent {
    pub game: Entity,
}

impl MapEntities for LeaveGameEvent {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.game = entity_mapper.map_entity(self.game);
    }
}
