use bevy::prelude::{EventReader, Query, ResMut, Resource};

use wildchess_game::{
    components::{Behavior, PieceKind, Promotable, Team},
    Movement, RequestPromotionEvent,
};

use crate::PieceIcon;

#[derive(Default, Resource)]
pub struct IntendedPromotion(pub Option<(Movement, Vec<(PieceIcon, Behavior)>)>);

pub fn read_promotions(
    mut intended_promotion: ResMut<IntendedPromotion>,
    mut promotion_reader: EventReader<RequestPromotionEvent>,
    promotable_query: Query<(&PieceKind, &Team, &Promotable)>,
) {
    for event in promotion_reader.iter() {
        let entity = event.0.entity;
        if let Ok((piece, team, promotable)) = promotable_query.get(entity) {
            intended_promotion.0 = Some((
                event.0.clone(),
                promotable
                    .behaviors
                    .iter()
                    .map(move |behavior| {
                        (
                            PieceIcon::new_wild(*piece, &behavior, *team),
                            behavior.clone(),
                        )
                    })
                    .collect(),
            ));
        }
    }
}
