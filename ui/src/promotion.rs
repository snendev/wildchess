use bevy::prelude::{EventReader, Query, ResMut, Resource};

use chess_gameplay::{
    chess::{
        pieces::{Behavior, King, Promotable},
        team::Team,
    },
    Movement, RequestPromotionEvent,
};

use crate::PieceIcon;

#[derive(Default, Resource)]
pub struct IntendedPromotion(pub Option<(Movement, Vec<(PieceIcon, Behavior)>)>);

pub fn read_promotions(
    mut intended_promotion: ResMut<IntendedPromotion>,
    mut promotion_reader: EventReader<RequestPromotionEvent>,
    promotable_query: Query<(&Team, &Promotable, Option<&King>)>,
) {
    for event in promotion_reader.iter() {
        let entity = event.0.entity;
        if let Ok((team, promotable, king)) = promotable_query.get(entity) {
            intended_promotion.0 = Some((
                event.0.clone(),
                promotable
                    .behaviors
                    .iter()
                    .map(move |behavior| {
                        (
                            PieceIcon::wild_svg(behavior, *team, king.is_some()),
                            behavior.clone(),
                        )
                    })
                    .collect(),
            ));
        }
    }
}
