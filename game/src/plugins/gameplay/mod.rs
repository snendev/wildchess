use itertools::Itertools;

use bevy::prelude::{Added, App, Commands, IntoSystemConfigs, Plugin, Query, Update};

use crate::{
    components::{Board, PieceBundle, Team},
    IssueMoveEvent, IssuePromotionEvent, RequestPromotionEvent, TurnEvent,
};

mod capture;
mod targets;
mod turns;

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
        app.add_event::<TurnEvent>()
            .add_event::<IssueMoveEvent>()
            .add_event::<IssuePromotionEvent>()
            .add_event::<RequestPromotionEvent>()
            .add_systems(
                Update,
                (
                    turns::detect_turn,
                    turns::execute_turn,
                    capture::capture_pieces,
                    targets::calculate_targets,
                    turns::end_turn,
                )
                    .chain(),
            )
            .add_systems(Update, initialize_board);
    }
}
