use bevy::prelude::{Entity, EventReader, Query, ResMut, Resource};

use games::{
    chess::{
        pieces::{Action, Mutation, PieceDefinition, Royal},
        team::Team,
    },
    RequestMutationEvent,
};

use crate::PieceIcon;

#[derive(Default, Resource)]
pub struct IntendedMutation(pub Option<(Entity, Action, Vec<(PieceIcon, PieceDefinition)>)>);

pub fn read_mutation_options(
    mut intended_mutation: ResMut<IntendedMutation>,
    mut mutation_reader: EventReader<RequestMutationEvent>,
    piece_query: Query<(&Mutation, &Team, Option<&Royal>)>,
) {
    for event in mutation_reader.iter() {
        let entity = event.0;
        if let Ok((mutation, team, maybe_royal)) = piece_query.get(entity) {
            intended_mutation.0 = Some((
                entity,
                event.1.clone(),
                mutation
                    .to_piece
                    .iter()
                    .map(move |option| {
                        // TODO: don't construct this unnecessarily
                        let empty = Vec::new();
                        let patterns_to_display = if let Some(behavior) = &option.behaviors.pattern
                        {
                            &behavior.patterns
                        } else if let Some(behavior) = &option.behaviors.relay {
                            &behavior.patterns
                        } else {
                            &empty
                        };
                        (
                            PieceIcon::wild_svg(patterns_to_display, *team, maybe_royal.is_some()),
                            option.clone(),
                        )
                    })
                    .collect(),
            ));
        }
    }
}
