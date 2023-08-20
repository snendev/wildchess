use itertools::Itertools;

use bevy::prelude::{Added, App, Commands, IntoSystemConfigs, Plugin, Query, Update};

use crate::{
    components::{Board, PieceBundle, Team},
    Movement, PieceEvent, Promotion, RequestPromotion,
};

mod capture;
mod movement;
mod targets;

fn initialize_board(mut commands: Commands, query: Query<&Board, Added<Board>>) {
    for Board {
        pieces,
        size: _size,
    } in query.iter()
    {
        for (config, start_positions) in pieces.0.clone().into_iter() {
            let promotable = config.promotable.clone();
            let pieces = start_positions
                .into_iter()
                .cartesian_product([Team::White, Team::Black].into_iter())
                .map(move |(position, team)| {
                    PieceBundle::from_configuration(&config, position, team)
                });
            if let Some(promotable) = promotable {
                commands.spawn_batch(pieces.map(move |piece| (piece, promotable.clone())));
            } else {
                commands.spawn_batch(pieces);
            }
        }
    }
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PieceEvent<Movement>>()
            .add_event::<PieceEvent<RequestPromotion>>()
            .add_event::<PieceEvent<Promotion>>()
            .add_systems(
                Update,
                (
                    movement::move_pieces,
                    movement::promote_pieces,
                    capture::capture_pieces,
                    targets::calculate_targets,
                )
                    .chain(),
            )
            .add_systems(Update, initialize_board);
    }
}
