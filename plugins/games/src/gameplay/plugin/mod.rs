use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{
    on_event, Added, Component, Condition, IntoSystemConfigs, IntoSystemSetConfigs, Query,
    SystemSet,
};

use chess::{
    actions::Actions,
    behavior::{BehaviorsPlugin, BehaviorsSystems, MimicBehavior, PatternBehavior, RelayBehavior},
    pieces::Position,
    ChessPlugin,
};

use bevy_replicon::prelude::*;

mod events;
pub use events::{RequestTurnEvent, RequireMutationEvent, TurnEvent};
use systems::detect_turn;

use crate::{
    components::{
        ActionHistory, AntiGame, Atomic, ClockConfiguration, Crazyhouse, Game, GameBoard, GameOver,
        HasTurn, History, InGame, LastMove, Ply, WinCondition,
    },
    MatchmakingSystems,
};

mod systems;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub enum GameSystems {
    All,
    DetectGameover,
    TrackHistory,
    DetectTurn,
    ExecuteTurn,
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<RepliconCorePlugin>() {
            app.add_plugins((RepliconCorePlugin, ParentSyncPlugin));
        }
        app.add_plugins((
            ChessPlugin,
            BehaviorsPlugin::from_input_system(systems::last_action),
            crate::ClockPlugin,
        ))
        .configure_sets(
            Update,
            BehaviorsSystems
                .run_if(any_with_component_added::<Actions>().or_else(on_event::<TurnEvent>())),
        )
        .configure_sets(Update, GameSystems::All.before(BehaviorsSystems))
        // todo doesn't really belong here, but useful for now
        .configure_sets(Update, GameSystems::All.after(MatchmakingSystems))
        .add_mapped_client_event::<RequestTurnEvent>(ChannelKind::Ordered)
        .add_mapped_server_event::<RequireMutationEvent>(ChannelKind::Ordered)
        .add_event::<TurnEvent>()
        .replicate::<Ply>()
        .replicate::<HasTurn>()
        .replicate_mapped::<InGame>()
        .replicate::<Game>()
        .replicate::<GameOver>()
        .replicate::<LastMove>()
        .replicate::<GameBoard>()
        .replicate::<Atomic>()
        .replicate::<Crazyhouse>()
        .replicate::<AntiGame>()
        .replicate::<WinCondition>()
        .replicate::<ClockConfiguration>()
        .replicate_mapped::<ActionHistory>()
        .replicate::<History<Position>>()
        .replicate::<History<PatternBehavior>>()
        .replicate::<History<MimicBehavior>>()
        .replicate::<History<RelayBehavior>>()
        .configure_sets(
            Update,
            (
                GameSystems::TrackHistory,
                GameSystems::DetectTurn,
                GameSystems::ExecuteTurn.run_if(on_event::<TurnEvent>()),
                GameSystems::DetectGameover.run_if(on_event::<TurnEvent>()),
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
                History::<MimicBehavior>::track_component_system,
                History::<RelayBehavior>::track_component_system,
            )
                .chain()
                .in_set(GameSystems::TrackHistory),
        )
        .add_systems(Update, detect_turn.in_set(GameSystems::DetectTurn))
        .add_systems(
            Update,
            (
                systems::execute_turn_movement,
                systems::execute_turn_mutations,
                systems::set_last_move,
                systems::end_turn,
                // TODO: double check how history behaves wrt TurnEvent and system ordering
                systems::track_turn_history,
            )
                .chain()
                .in_set(GameSystems::ExecuteTurn),
        )
        .add_systems(
            Update,
            systems::detect_gameover.in_set(GameSystems::DetectGameover),
        );

        #[cfg(feature = "reflect")]
        app.register_type::<InGame>()
            .register_type::<GameBoard>()
            .register_type::<WinCondition>()
            .register_type::<Ply>()
            .register_type::<LastMove>()
            .register_type::<ClockConfiguration>()
            .register_type::<ActionHistory>();
    }
}

pub fn any_with_component_added<T: Component>() -> impl FnMut(Query<(), Added<T>>) -> bool {
    move |query: Query<(), Added<T>>| query.iter().count() > 0
}

#[cfg(test)]
mod tests {
    use bevy_ecs::{entity::Entity, event::Events, world::World};
    use chess::board::Square;
    use layouts::RandomWildLayout;

    use crate::components::{GameBoard, GameSpawner, PieceSet, WinCondition};

    use super::*;

    fn get_piece_actions<'a>(
        world: &'a mut World,
        piece_square: Square,
    ) -> Option<(Entity, Position, Actions)> {
        let mut query = world.query::<(&Position, &Actions)>();
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
        let mut query = world.query::<(Entity, &Position, &Actions)>();
        let (piece, _, actions) = query
            .iter(&world)
            .find(|(_, position, _)| position.0 == piece_square)
            .unwrap();
        let (_, action) = actions
            .0
            .iter()
            .find(|(square, _)| **square == target_square)
            .unwrap();
        let action = action.clone();

        RequestTurnEvent::new(piece, action.clone())
    }

    #[test]
    fn test_lifecycle() {
        let mut app = bevy_app::App::default();
        app.add_plugins(bevy_core::TaskPoolPlugin::default());
        app.add_plugins(bevy_core::TypeRegistrationPlugin);
        app.add_plugins(bevy_core::FrameCountPlugin);
        app.add_plugins(bevy_time::TimePlugin);
        app.add_plugins(bevy_app::ScheduleRunnerPlugin::default());
        app.add_plugins(RepliconCorePlugin);

        app.add_plugins(GameplayPlugin);

        // TODO: use blueprints?
        let game_spawner = GameSpawner::new_game(
            GameBoard::Chess,
            PieceSet(RandomWildLayout::pieces()),
            WinCondition::RoyalCapture,
        );
        app.world_mut().spawn((
            game_spawner.game,
            game_spawner.board,
            game_spawner.win_condition,
        ));

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
