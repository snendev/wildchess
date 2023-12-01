use bevy::prelude::{
    on_event, Added, App, Component, Condition, IntoSystemConfigs, IntoSystemSetConfigs, Plugin,
    PostUpdate, Query, Update,
};

use chess::{
    actions::Actions,
    behavior::{BehaviorsPlugin, BehaviorsSet, MimicBehavior, PatternBehavior, RelayBehavior},
    pieces::Position,
    ChessTypesPlugin,
};

mod events;
pub use events::{IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent};

use crate::components::History;

use self::events::GameoverEvent;

mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChessTypesPlugin,
            BehaviorsPlugin::from_input_system(systems::last_action),
        ))
        .configure_sets(
            PostUpdate,
            BehaviorsSet
                .run_if(any_with_component_added::<Actions>().or_else(on_event::<TurnEvent>())),
        )
        .add_event::<TurnEvent>()
        .add_event::<IssueMoveEvent>()
        .add_event::<IssueMutationEvent>()
        .add_event::<RequestMutationEvent>()
        .add_event::<GameoverEvent>()
        .add_systems(
            Update,
            (
                systems::detect_gameover.run_if(on_event::<TurnEvent>()),
                systems::log_gameover_events.run_if(on_event::<TurnEvent>()),
                // TODO: should this only be in UI? probably!
                History::<Position>::track_component_system,
                History::<PatternBehavior>::track_component_system,
                History::<MimicBehavior>::track_component_system,
                History::<RelayBehavior>::track_component_system,
                // TODO: stop playing after gameover
                systems::detect_turn,
                systems::execute_turn_movement.run_if(on_event::<TurnEvent>()),
                systems::execute_turn_mutations.run_if(on_event::<TurnEvent>()),
                systems::end_turn.run_if(on_event::<TurnEvent>()),
                systems::track_turn_history.run_if(on_event::<TurnEvent>()),
                systems::tick_clocks,
            )
                .chain(),
        )
        .add_systems(Update, systems::spawn_game_entities);
    }
}

pub fn any_with_component_added<T: Component>() -> impl FnMut(Query<(), Added<T>>) -> bool {
    move |query: Query<(), Added<T>>| query.iter().count() > 0
}
