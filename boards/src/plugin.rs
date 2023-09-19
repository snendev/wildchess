use bevy::prelude::{
    on_event, Added, App, Commands, Component, Event, EventReader, IntoSystemConfigs, Plugin,
    Query, Update,
};
use chess::board::Board;

use crate::{
    classical::{ClassicalIdentity, ClassicalLayout},
    wild::WildLayout,
};

#[derive(Clone, Copy, Debug, Default, Event)]
pub struct BuildWildBoardEvent;

#[derive(Clone, Copy, Debug, Default, Event)]
pub struct BuildClassicalBoardEvent;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildWildBoardEvent>()
            .add_event::<BuildClassicalBoardEvent>()
            .add_systems(
                Update,
                (
                    configure_wild_boards.run_if(on_event::<BuildWildBoardEvent>()),
                    configure_classical_boards.run_if(on_event::<BuildClassicalBoardEvent>()),
                ),
            );

        #[cfg(debug_assertions)]
        app.register_type::<ClassicalIdentity>();
    }
}

fn spawn_game_entities(mut commands: Commands, query: Query<&Game, Added<Game>>) {
    for added_game in query.iter() {
        let board = match added_game {
            Game::Chess | Game::WildChess | Game::SuperRelayChess => Board::chess_board(),
        };
        commands.spawn(board);
        // TODO: spawn pieces
    }
}

fn configure_wild_boards(mut commands: Commands, mut reader: EventReader<BuildWildBoardEvent>) {
    for _ in reader.iter() {
        for piece in WildLayout::pieces() {
            piece.spawn(&mut commands);
        }
    }
}

fn configure_classical_boards(
    mut commands: Commands,
    mut reader: EventReader<BuildClassicalBoardEvent>,
) {
    for _ in reader.iter() {
        for piece in ClassicalLayout::pieces() {
            piece.spawn(&mut commands);
        }
    }
}

#[derive(Clone, Component, Debug, Default)]
pub enum Game {
    Chess,
    #[default]
    WildChess,
    SuperRelayChess,
    // Checkers, // TODO
}
