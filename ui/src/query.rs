use bevy::{ecs::query::WorldQuery, prelude::Entity};

use games::chess::{
    actions::Actions,
    behavior::{MimicBehavior, PatternBehavior, RelayBehavior},
    board::OnBoard,
    pieces::{Mutation, Position},
    team::Team,
};

use crate::PieceIcon;

#[derive(WorldQuery)]
pub struct BehaviorsQuery {
    pub behavior: Option<&'static PatternBehavior>,
    pub relay_behavior: Option<&'static RelayBehavior>,
    pub mimic_behavior: Option<&'static MimicBehavior>,
    pub mutation: Option<&'static Mutation>,
}

#[derive(WorldQuery)]
pub struct PieceQuery {
    pub entity: Entity,
    pub on_board: &'static OnBoard,
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
    pub on_board: &'a OnBoard,
    pub position: &'a Position,
    pub team: &'a Team,
    pub actions: &'a Actions,
    pub pattern_behavior: Option<&'a PatternBehavior>,
    pub relay_behavior: Option<&'a RelayBehavior>,
    pub mimic_behavior: Option<&'a MimicBehavior>,
    pub mutation: Option<&'a Mutation>,
    pub icon: Option<&'a PieceIcon>,
}

impl<'a> From<PieceQueryItem<'a>> for PieceData<'a> {
    fn from(piece: PieceQueryItem<'a>) -> Self {
        PieceData {
            entity: piece.entity,
            on_board: piece.on_board,
            position: piece.position,
            team: piece.team,
            actions: piece.actions,
            pattern_behavior: piece.behavior,
            relay_behavior: piece.relay_behavior,
            mimic_behavior: piece.mimic_behavior,
            mutation: piece.mutation,
            icon: piece.icon,
        }
    }
}
