use bevy::{ecs::query::WorldQuery, prelude::Entity};

use games::{
    chess::{
        actions::Actions,
        behavior::{MimicBehavior, PatternBehavior, RelayBehavior},
        board::OnBoard,
        pieces::{Mutation, Position},
        team::Team,
    },
    components::{History, InGame, Ply},
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
    pub in_game: &'static InGame,
    pub on_board: &'static OnBoard,
    pub position: Option<&'static Position>,
    pub team: &'static Team,
    pub actions: &'static Actions,
    pub behavior: Option<&'static PatternBehavior>,
    pub relay_behavior: Option<&'static RelayBehavior>,
    pub mimic_behavior: Option<&'static MimicBehavior>,
    pub mutation: Option<&'static Mutation>,
    pub icon: Option<&'static PieceIcon<'static>>,
    pub position_history: &'static History<Position>,
    pub behavior_history: Option<&'static History<PatternBehavior>>,
    pub relay_behavior_history: Option<&'static History<RelayBehavior>>,
    pub mimic_behavior_history: Option<&'static History<MimicBehavior>>,
    pub icon_history: Option<&'static History<PieceIcon<'static>>>,
}

pub struct PieceData<'a> {
    pub entity: Entity,
    pub in_game: &'a InGame,
    pub on_board: &'a OnBoard,
    pub team: &'a Team,
    pub actions: &'a Actions,
    pub position: Option<&'a Position>,
    pub pattern_behavior: Option<&'a PatternBehavior>,
    pub relay_behavior: Option<&'a RelayBehavior>,
    pub mimic_behavior: Option<&'a MimicBehavior>,
    pub mutation: Option<&'a Mutation>,
    pub icon: Option<&'a PieceIcon<'a>>,
}

impl<'a> From<PieceQueryItem<'a>> for PieceData<'a> {
    fn from(piece: PieceQueryItem<'a>) -> Self {
        PieceData {
            entity: piece.entity,
            in_game: piece.in_game,
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

impl<'a> PieceQueryItem<'a> {
    pub fn to_historical_piece_data(&self, ply: &Ply) -> PieceData<'a> {
        PieceData {
            entity: self.entity,
            in_game: self.in_game,
            on_board: self.on_board,
            position: self.position_history.get_previous_nearest(ply),
            team: self.team,
            actions: self.actions,
            pattern_behavior: self
                .behavior_history
                .and_then(|behavior| behavior.get_previous_nearest(ply)),
            relay_behavior: self
                .relay_behavior_history
                .and_then(|behavior| behavior.get_previous_nearest(ply)),
            mimic_behavior: self
                .mimic_behavior_history
                .and_then(|behavior| behavior.get_previous_nearest(ply)),
            mutation: self.mutation,
            icon: self
                .icon_history
                .and_then(|icon| icon.get_previous_nearest(ply)),
        }
    }
}
