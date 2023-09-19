use bevy::prelude::{Added, App, Commands, Plugin, Query, Update};
use chess::board::Board;

use crate::{
    classical::{ClassicalIdentity, ClassicalLayout},
    knight_relay::KnightRelayLayout,
    super_relay::SuperRelayLayout,
    wild::WildLayout,
    Game,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_game_entities);

        #[cfg(debug_assertions)]
        app.register_type::<ClassicalIdentity>();
    }
}

fn spawn_game_entities(mut commands: Commands, query: Query<&Game, Added<Game>>) {
    for added_game in query.iter() {
        let board = match added_game {
            Game::Chess | Game::WildChess | Game::KnightRelayChess | Game::SuperRelayChess => {
                Board::chess_board()
            }
        };
        commands.spawn(board);
        match added_game {
            Game::Chess => ClassicalLayout::spawn_pieces(&mut commands),
            Game::WildChess => WildLayout::spawn_pieces(&mut commands),
            Game::KnightRelayChess => KnightRelayLayout::spawn_pieces(&mut commands),
            Game::SuperRelayChess => SuperRelayLayout::spawn_pieces(&mut commands),
        }
    }
}
