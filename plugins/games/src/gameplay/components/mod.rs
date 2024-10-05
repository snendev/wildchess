use serde::{Deserialize, Serialize};

use bevy::{
    ecs::entity::MapEntities,
    prelude::{Component, Entity, EntityMapper, Reflect, With, Without},
};

use bevy_replicon::prelude::ClientId;

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, CurrentTurn, Game, GameBoard, PieceSet,
    SpawnGame, WinCondition,
};
mod turns;
pub use turns::{ActionHistory, History, Ply};

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Client {
    pub id: ClientId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct InGame(pub Entity);

impl MapEntities for InGame {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = mapper.map_entity(self.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct GameOver {
    winner: chess::team::Team,
}

impl GameOver {
    pub fn new(winner: chess::team::Team) -> Self {
        Self { winner }
    }

    pub fn winner(&self) -> &chess::team::Team {
        &self.winner
    }
}

pub type IsActiveGame = (With<Game>, Without<GameOver>);
