use bevy::{
    prelude::{Changed, Entity, Or, Query},
    utils::HashMap,
};

use chess::{
    pieces::{Behavior, Position, Targets},
    square::Square,
    team::Team,
};

type WithUpdatedPiece = Or<(Changed<Position>, Changed<Behavior>)>;
pub fn calculate_targets(
    mut piece_query: Query<(Entity, &Behavior, &Position, &Team, &mut Targets)>,
    update_query: Query<Entity, WithUpdatedPiece>,
) {
    if update_query.is_empty() {
        return;
    }

    let pieces: HashMap<Square, (Entity, Team)> = piece_query
        .iter()
        .map(|(entity, _, position, team, _)| (position.0, (entity, *team)))
        .collect();

    for (_, behavior, position, team, mut vision) in piece_query.iter_mut() {
        vision.set(behavior.search(&position.0, *team, &pieces));
    }
}

pub fn _calculate_psychic_targets(
    mut piece_query: Query<(Entity, &Behavior, &Position, &Team, &mut Targets)>,
    update_query: Query<Entity, Changed<Position>>,
) {
    if update_query.is_empty() {
        return;
    }

    todo!();
    let pieces: HashMap<Square, (Entity, Team)> = piece_query
        .iter()
        .map(|(entity, _, position, team, _)| (position.0, (entity, *team)))
        .collect();

    for (_, behavior, position, team, mut vision) in piece_query.iter_mut() {
        vision.set(behavior.search(&position.0, *team, &pieces));
    }
}
