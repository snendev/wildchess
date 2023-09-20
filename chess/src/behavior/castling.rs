use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    board::{Board, Square},
    pieces::{Action, Actions, Orientation, Position},
    team::Team,
};

use super::Behavior;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CastlingBehavior {
    has_moved: bool,
}

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CastlingTarget {
    has_moved: bool,
}

// Enable performing whatever Pattern was executed in the last turn
impl Behavior for CastlingBehavior {
    fn add_actions_system(
        In(last_action): In<Option<Action>>,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Option<&CastlingBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut Actions,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, position, _, team, _)| (position.0, *team))
            .collect();

        for (castling, position, orientation, team, mut actions) in piece_query.iter_mut() {
            if castling.is_some() {
                // TODO
            }
        }
    }
}
