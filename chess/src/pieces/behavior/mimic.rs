use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    board::{common::chess_board, Square},
    pieces::{Action, Actions, Orientation, Position},
    team::Team,
};

use super::Behavior;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
// WIP: Too many problems around target squares right now
// For example, RotationBehavior currently does not allow rotating towards the right
// from the H file
// Multiple actions on the same square are also not yet supported
pub struct MimicBehavior;

// Enable performing whatever Pattern was executed in the last turn
impl Behavior for MimicBehavior {
    fn add_actions_system(
        In(last_action): In<Option<Action>>,
        mut piece_query: Query<(
            Option<&MimicBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut Actions,
        )>,
    ) {
        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, position, _, team, _)| (position.0, *team))
            .collect();

        if let Some(last_action) = last_action {
            for (mimic, position, orientation, team, mut actions) in piece_query.iter_mut() {
                if mimic.is_some() {
                    actions.extend(Actions::new(last_action.using_pattern.search(
                        &position.0,
                        &orientation,
                        team,
                        &chess_board().size,
                        &pieces,
                        Some(&last_action),
                    )))
                }
            }
        }
    }
}
