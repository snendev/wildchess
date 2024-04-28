use bevy::prelude::{EventReader, Query, ResMut, Resource};

use games::{
    chess::{
        pieces::{Mutation, PieceDefinition, Royal},
        team::Team,
    },
    RequestMutationEvent,
};
use wild_icons::PieceIcon;

#[allow(clippy::type_complexity)]
#[derive(Default, Resource)]
pub struct IntendedMutation(pub Option<(RequestMutationEvent, Vec<(PieceIcon, PieceDefinition)>)>);

pub fn read_mutation_options(
    mut intended_mutation: ResMut<IntendedMutation>,
    mut mutation_reader: EventReader<RequestMutationEvent>,
    piece_query: Query<(&Mutation, &Team, Option<&Royal>)>,
) {
    for event in mutation_reader.read() {
        let entity = event.piece;
        if let Ok((mutation, team, maybe_royal)) = piece_query.get(entity) {
            intended_mutation.0 = Some((
                event.clone(),
                mutation
                    .to_piece
                    .iter()
                    .enumerate()
                    .map(move |(index, option)| {
                        (
                            PieceIcon::from_behaviors(
                                option.identity,
                                format!("temp-{:?}", index),
                                option.behaviors.pattern.as_ref(),
                                option.behaviors.relay.as_ref(),
                                *team,
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
