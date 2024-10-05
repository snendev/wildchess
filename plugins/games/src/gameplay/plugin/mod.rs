use bevy::prelude::{
    Added, App, Changed, Component, Condition, IntoSystemConfigs, IntoSystemSetConfigs, Plugin,
    Query, SystemSet, Update,
};

use chess::{
    actions::Actions,
    behavior::{BehaviorsPlugin, BehaviorsSystems, PatternBehavior, RelayBehavior},
    pieces::Position,
    ChessPlugin,
};

use bevy_replicon::prelude::*;

use crate::{
    components::{
        ActionHistory, AntiGame, Atomic, ClockConfiguration, Crazyhouse, Game, GameBoard, GameOver,
        History, InGame, Ply, WinCondition,
    },
    ClockPlugin, MatchmakingSystems,
};

use super::components::{Client, CurrentTurn, Player, SpawnGame};

mod events;
pub use events::*;
mod systems;
mod turns;
use turns::PlayTurn;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub enum GameSystems {
    All,
    TriggerTurn,
    TrackHistory,
    DetectGameover,
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ChessPlugin, BehaviorsPlugin, ClockPlugin))
            .configure_sets(
                Update,
                BehaviorsSystems.run_if(any_with_component_added::<Actions>().or_else(
                    // TODO: do this some other way
                    any_with_component_changed::<CurrentTurn>(),
                )),
            )
            .configure_sets(Update, GameSystems::All.before(BehaviorsSystems))
            // todo doesn't really belong here, but useful for now
            .configure_sets(Update, GameSystems::All.after(MatchmakingSystems))
            .add_mapped_client_event::<RequestTurnEvent>(ChannelKind::Ordered)
            .add_mapped_server_event::<RequireMutationEvent>(ChannelKind::Ordered)
            .replicate::<Ply>()
            .replicate_mapped::<InGame>()
            .replicate::<Game>()
            .replicate::<Client>()
            .replicate::<Player>()
            .replicate::<CurrentTurn>()
            .replicate::<GameOver>()
            .replicate::<GameBoard>()
            .replicate::<Atomic>()
            .replicate::<Crazyhouse>()
            .replicate::<AntiGame>()
            .replicate::<WinCondition>()
            .replicate::<ClockConfiguration>()
            .replicate_mapped::<ActionHistory>()
            .replicate::<History<Position>>()
            .replicate::<History<PatternBehavior>>()
            .replicate::<History<RelayBehavior>>()
            .configure_sets(
                Update,
                (
                    GameSystems::TriggerTurn,
                    GameSystems::TrackHistory,
                    GameSystems::DetectGameover,
                )
                    .chain()
                    .in_set(GameSystems::All),
            )
            .configure_sets(Update, GameSystems::All.run_if(has_authority))
            .add_systems(
                Update,
                (
                    // TODO: make an independent lib for this stuff & maybe UI/utils
                    History::<Position>::track_component_system,
                    History::<PatternBehavior>::track_component_system,
                    History::<RelayBehavior>::track_component_system,
                )
                    .chain()
                    .in_set(GameSystems::TrackHistory),
            )
            .add_systems(
                Update,
                systems::trigger_turns.in_set(GameSystems::TriggerTurn),
            )
            .add_systems(
                Update,
                // TODO: double check how history behaves wrt PlayTurn and system ordering
                systems::detect_gameover.in_set(GameSystems::DetectGameover),
            );

        app.observe(SpawnGame::observer);
        app.observe(PlayTurn::observer);

        app.register_type::<InGame>()
            .register_type::<GameBoard>()
            .register_type::<WinCondition>()
            .register_type::<Ply>()
            .register_type::<ClockConfiguration>()
            .register_type::<ActionHistory>();
    }
}

pub fn any_with_component_added<T: Component>() -> impl FnMut(Query<(), Added<T>>) -> bool {
    move |query: Query<(), Added<T>>| query.iter().count() > 0
}

pub fn any_with_component_changed<T: Component>() -> impl FnMut(Query<(), Changed<T>>) -> bool {
    move |query: Query<(), Changed<T>>| query.iter().count() > 0
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, ScheduleRunnerPlugin};
    use bevy::core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
    use bevy::ecs::prelude::{Entity, Events, World};
    use bevy::time::TimePlugin;
    use bevy_replicon::core::RepliconCorePlugin;
    use chess::board::Square;
    use layouts::RandomWildLayout;

    use crate::components::PieceSet;

    use super::*;

    fn get_piece_actions<'a>(
        world: &'a mut World,
        piece_square: Square,
    ) -> Option<(Entity, Position, Actions)> {
        let mut query = world.query::<(&Position, &Actions)>();
        eprintln!("{:?}", query.iter(&world).collect::<Vec<_>>());
        let (_, actions) = query
            .iter(&world)
            .find(|(position, _)| position.0 == piece_square)
            .unwrap();
        eprintln!(
            "{:?}",
            actions.0.iter().map(|action| *action.0).collect::<Vec<_>>()
        );

        let mut query = world.query::<(Entity, &Position, &Actions)>();
        query
            .iter(&world)
            .find(|(_, position, _)| position.0 == piece_square)
            .map(|(entity, position, actions)| (entity, position.clone(), actions.clone()))
    }

    fn create_move_event<'a>(
        world: &'a mut World,
        piece_square: Square,
        target_square: Square,
    ) -> RequestTurnEvent {
        let mut query = world.query::<(Entity, &Position, &Actions, &InGame)>();
        let (piece, _, actions, in_game) = query
            .iter(&world)
            .find(|(_, position, _, _)| position.0 == piece_square)
            .unwrap();
        let (_, action) = actions
            .0
            .iter()
            .find(|(square, _)| **square == target_square)
            .unwrap();
        let action = action.clone();

        RequestTurnEvent::new(piece, in_game.0, action.clone())
    }

    #[test]
    fn test_lifecycle() {
        let mut app = App::default();
        app.add_plugins(TaskPoolPlugin::default());
        app.add_plugins(TypeRegistrationPlugin);
        app.add_plugins(FrameCountPlugin);
        app.add_plugins(TimePlugin);
        app.add_plugins(ScheduleRunnerPlugin::default());
        app.add_plugins(RepliconCorePlugin);

        app.add_plugins(GameplayPlugin);

        app.world_mut()
            .trigger(SpawnGame::new(PieceSet(RandomWildLayout::pieces())));

        app.update();

        let (piece, _, actions) =
            get_piece_actions(app.world_mut(), "a2".try_into().unwrap()).unwrap();
        eprintln!("Actions on spawn for {:?}: {:?}", piece, actions.0);

        let move_event = create_move_event(
            app.world_mut(),
            "a2".try_into().unwrap(),
            "a3".try_into().unwrap(),
        );
        eprintln!(
            "Move event for {:?}: {:?}",
            move_event.piece, move_event.action
        );

        let mut move_events = app.world_mut().resource_mut::<Events<RequestTurnEvent>>();
        move_events.send(move_event);

        app.update();
        let (_, _, actions) = get_piece_actions(app.world_mut(), "a3".try_into().unwrap()).unwrap();
        eprintln!("Actions after move: {:?}", actions.0);
    }
}
