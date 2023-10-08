use bevy::prelude::{
    on_event, Added, App, Component, Condition, IntoSystemConfigs, IntoSystemSetConfig, Plugin,
    PostUpdate, Query, Update,
};

use chess::{
    actions::Actions,
    behavior::{BehaviorsPlugin, BehaviorsSet},
    ChessTypesPlugin,
};

mod events;
pub use events::{IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent};

use self::events::GameoverEvent;

mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChessTypesPlugin,
            BehaviorsPlugin::from_input_system(systems::last_action),
        ))
        .configure_set(
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
                // TODO: stop playing after gameover
                systems::detect_turn,
                systems::execute_turn_movement.run_if(on_event::<TurnEvent>()),
                systems::execute_turn_mutations.run_if(on_event::<TurnEvent>()),
                systems::end_turn.run_if(on_event::<TurnEvent>()),
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
