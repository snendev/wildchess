use bevy::prelude::{EventReader, Query, ResMut, Resource};

use chess_gameplay::{
    chess::pieces::{Mutation, PieceDefinition, Royal},
    Movement, RequestMutationEvent,
};

use crate::PieceIcon;

#[derive(Default, Resource)]
pub struct IntendedMutation(pub Option<(Movement, Vec<(PieceIcon, PieceDefinition)>)>);

pub fn read_mutation_options(
    mut intended_mutation: ResMut<IntendedMutation>,
    mut mutation_reader: EventReader<RequestMutationEvent>,
    piece_query: Query<(&Mutation, Option<&Royal>)>,
) {
    for event in mutation_reader.iter() {
        let entity = event.0.entity;
        if let Ok((mutation, maybe_royal)) = piece_query.get(entity) {
            intended_mutation.0 = Some((
                event.0.clone(),
                mutation
                    .options
                    .iter()
                    .map(move |option| {
                        (
                            PieceIcon::wild_svg(
                                &option.behavior,
                                option.team,
                                maybe_royal.is_some(),
                            ),
                            option.clone(),
                        )
                    })
                    .collect(),
            ));
        }
    }
}
