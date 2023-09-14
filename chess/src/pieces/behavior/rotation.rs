use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    board::common::chess_board,
    pieces::{Action, Actions, Orientation, Pattern, Position, Scanner},
    team::Team,
};

use super::Behavior;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
// WIP: Too many problems around target squares right now
// For example, RotationBehavior currently does not allow rotating towards the right
// from the H file
// Multiple actions on the same square are also not yet supported
pub struct RotationBehavior;

// Enable taking a turn to rotate a piece's orientation
impl Behavior for RotationBehavior {
    fn add_actions_system(
        In(_): In<Option<Action>>,
        mut piece_query: Query<(
            Option<&RotationBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut Actions,
        )>,
    ) {
        for (_, position, orientation, team, mut actions) in piece_query.iter_mut() {
            let rotation_actions = Actions::new(
                orientation
                    .other_orientations()
                    .into_iter()
                    .flat_map(|orientation: Orientation| {
                        Scanner::forward()
                            .scan(
                                &position.0,
                                orientation,
                                team,
                                &chess_board().size,
                                &HashMap::default(),
                            )
                            .into_iter()
                            .map(|scan_target| scan_target.target)
                            .map(move |square| {
                                (
                                    square,
                                    Action::movement(
                                        position.0,
                                        orientation,
                                        vec![],
                                        Pattern::default(),
                                    ),
                                )
                            })
                    })
                    .collect(),
            );
            actions.extend(rotation_actions);
        }
    }
}
