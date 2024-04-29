use bevy_app::prelude::{App, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::{
    on_event, Added, Component, Condition, IntoSystemConfigs, IntoSystemSetConfigs, Query,
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

#[cfg(test)]
mod tests {
    use bevy_ecs::{entity::Entity, event::Events, world::World};
    use chess::board::Square;

    use crate::components::{GameBoard, GameSpawner, WinCondition};

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
    ) -> IssueMoveEvent {
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

        IssueMoveEvent {
            piece,
            action: action.clone(),
        }
    }

    #[test]
    fn test_lifecycle() {
        let mut app = bevy_app::App::default();
        app.add_plugins(bevy_core::TaskPoolPlugin::default());
        app.add_plugins(bevy_core::TypeRegistrationPlugin);
        app.add_plugins(bevy_core::FrameCountPlugin);
        app.add_plugins(bevy_time::TimePlugin);
        app.add_plugins(bevy_app::ScheduleRunnerPlugin::default());

        app.add_plugins(GameplayPlugin);

        // TODO: use blueprints?
        let game_spawner = GameSpawner::new_game(GameBoard::WildChess, WinCondition::RoyalCapture);
        app.world.spawn((
            game_spawner.game,
            game_spawner.board,
            game_spawner.win_condition,
        ));

        app.update();

        let (piece, _, actions) =
            get_piece_actions(&mut app.world, "a2".try_into().unwrap()).unwrap();
        eprintln!("Actions on spawn for {:?}: {:?}", piece, actions.0);

        let move_event = create_move_event(
            &mut app.world,
            "a2".try_into().unwrap(),
            "a3".try_into().unwrap(),
        );
        eprintln!(
            "Move event for {:?}: {:?}",
            move_event.piece, move_event.action
        );

        let mut move_events = app.world.resource_mut::<Events<IssueMoveEvent>>();
        move_events.send(move_event);

        app.update();
        let (_, _, actions) = get_piece_actions(&mut app.world, "a3".try_into().unwrap()).unwrap();
        eprintln!("Actions after move: {:?}", actions.0);
    }
}
