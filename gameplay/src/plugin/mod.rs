use bevy::prelude::{
    apply_deferred, on_event, Added, App, Commands, Component, Condition, IntoSystem,
    IntoSystemConfigs, Plugin, Query, Startup, Update,
};

use chess::{
    behavior::{Behavior, EnPassantBehavior, MimicBehavior, PatternBehavior, RelayBehavior},
    pieces::Actions,
    team::Team,
    ChessTypesPlugin,
};

use crate::{
    components::{PlayerBundle, Turn},
    IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent,
};

mod systems;

fn initialize_players(mut commands: Commands) {
    commands.spawn((PlayerBundle::new(Team::White), Turn));
    commands.spawn(PlayerBundle::new(Team::Black));
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChessTypesPlugin)
            .add_event::<TurnEvent>()
            .add_event::<IssueMoveEvent>()
            .add_event::<IssueMutationEvent>()
            .add_event::<RequestMutationEvent>()
            .add_systems(Startup, initialize_players)
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
                        systems::last_action.pipe(PatternBehavior::add_actions_system),
                        systems::last_action.pipe(EnPassantBehavior::add_actions_system),
                        systems::last_action.pipe(MimicBehavior::add_actions_system),
                        systems::last_action.pipe(RelayBehavior::add_actions_system),
                    )
                        .run_if(
                            any_with_component_added::<Actions>().or_else(on_event::<TurnEvent>()),
                        ),
                )
                    .chain(),
            );
    }
}

pub fn any_with_component_added<T: Component>() -> impl FnMut(Query<(), Added<T>>) -> bool {
    move |query: Query<(), Added<T>>| query.iter().count() > 0
}
