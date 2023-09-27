use bevy::prelude::{
    apply_deferred, on_event, Added, App, Component, Condition, IntoSystem, IntoSystemConfigs,
    Plugin, Query, Update,
};

use chess::{
    actions::Actions,
    behavior::{Behavior, EnPassantBehavior, MimicBehavior, PatternBehavior, RelayBehavior},
    ChessTypesPlugin,
};

mod events;
pub use events::{IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent};

use self::events::GameoverEvent;

mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChessTypesPlugin)
            .add_event::<TurnEvent>()
            .add_event::<IssueMoveEvent>()
            .add_event::<IssueMutationEvent>()
            .add_event::<RequestMutationEvent>()
            .add_event::<GameoverEvent>()
            .add_systems(
                Update,
                (
                    systems::detect_turn,
                    (
                        systems::execute_turn_movement,
                        systems::execute_turn_mutations,
                    ),
                    (systems::clear_actions, systems::end_turn).run_if(on_event::<TurnEvent>()),
                    apply_deferred,
                    (
                        systems::tick_clocks,
                        systems::detect_gameover.run_if(on_event::<TurnEvent>()),
                        systems::log_gameover_events.after(systems::detect_gameover),
                        (
                            // TODO: parallelize these by buffering actions inside Behavior first
                            // and then joining in a later system
                            // it could additionally be good to specify some
                            // BehaviorsPlugin<M, S: IntoSystem<Action, (), M>>
                            systems::last_action.pipe(PatternBehavior::add_actions_system),
                            systems::last_action.pipe(EnPassantBehavior::add_actions_system),
                            systems::last_action.pipe(MimicBehavior::add_actions_system),
                            systems::last_action.pipe(RelayBehavior::add_actions_system),
                        )
                            .run_if(
                                any_with_component_added::<Actions>()
                                    .or_else(on_event::<TurnEvent>()),
                            ),
                    ),
                )
                    .chain(),
            )
            .add_systems(Update, systems::spawn_game_entities);
    }
}

pub fn any_with_component_added<T: Component>() -> impl FnMut(Query<(), Added<T>>) -> bool {
    move |query: Query<(), Added<T>>| query.iter().count() > 0
}
