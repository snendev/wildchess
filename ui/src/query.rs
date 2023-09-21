use bevy::{ecs::query::WorldQuery, prelude::Entity};

use games::chess::{
    behavior::{MimicBehavior, PatternBehavior, RelayBehavior},
    pieces::{Actions, Mutation, Position},
    team::Team,
};

use crate::PieceIcon;

#[derive(WorldQuery)]
pub struct PieceQuery {
    pub entity: Entity,
    pub position: &'static Position,
    pub team: &'static Team,
    pub actions: &'static Actions,
    pub behavior: Option<&'static PatternBehavior>,
    pub relay_behavior: Option<&'static RelayBehavior>,
    pub mimic_behavior: Option<&'static MimicBehavior>,
    pub mutation: Option<&'static Mutation>,
    pub icon: Option<&'static PieceIcon>,
}

pub struct PieceData<'a> {
    pub entity: Entity,
    pub position: &'a Position,
    pub team: &'a Team,
    pub actions: &'a Actions,
    pub behavior: Option<&'a PatternBehavior>,
    pub relay_behavior: Option<&'a RelayBehavior>,
    pub mimic_behavior: Option<&'a MimicBehavior>,
    pub mutation: Option<&'a Mutation>,
    pub icon: Option<&'a PieceIcon>,
}

impl<'a> From<PieceQueryItem<'a>> for PieceData<'a> {
    fn from(piece: PieceQueryItem<'a>) -> Self {
        PieceData {
            entity: piece.entity,
            position: piece.position,
            team: piece.team,
            actions: piece.actions,
            behavior: piece.behavior,
            relay_behavior: piece.relay_behavior,
            mimic_behavior: piece.mimic_behavior,
            mutation: piece.mutation,
            icon: piece.icon,
        }
    }
}
