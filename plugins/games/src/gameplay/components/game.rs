use serde::{Deserialize, Serialize};

use bevy_core::Name;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::{
    event::Event,
    observer::Trigger,
    prelude::{Commands, Component, Entity},
};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use bevy_replicon::prelude::Replicated;

use chess::{
    board::{Rank, Square},
    team::Team,
};
use layouts::PieceSpecification;

use crate::Clock;

use super::{InGame, Player};

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
pub struct Game;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub enum GameBoard {
    #[default]
    Chess,
    // Shogi,    // TODO
    // Checkers, // TODO
}

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
pub struct PieceSet(pub Vec<PieceSpecification>);

impl From<Vec<PieceSpecification>> for PieceSet {
    fn from(pieces: Vec<PieceSpecification>) -> Self {
        Self(pieces)
    }
}

// A game rule specifying that captures result in an "explosion"
// additionally capturing on all squares in the region of the capture.
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct Atomic;

// A game rule specifying that players can place captured pieces
// on the board using a turn.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
pub struct Crazyhouse;

// A game rule specifying that the typical win condition results in a loss;
// Pieces must capture if they are able to.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
pub struct AntiGame;

// The set of win conditions for the board
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub enum WinCondition {
    // The game is won once all enemy Royal pieces are captured.
    RoyalCaptureAll,
    // The game is won once a single enemy Royal piece is captured.
    #[default]
    RoyalCapture,
    // The game is won once a Royal piece reaches a specific Rank.
    // (The Rank is local to the player's Orientation.)
    RaceToRank(Rank),
    //The game is won once a Royal piece reaches any of the given Squares.
    RaceToRegion(Vec<Square>),
}

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct ClockConfiguration {
    pub clock: Clock,
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct CurrentTurn(pub Team);

// TODO: revisit this API
// perhaps use the blueprints lib
#[derive(Clone, Default)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct SpawnGame {
    players: Option<(Entity, Entity)>,
    game: Game,
    board: GameBoard,
    turn: CurrentTurn,
    piece_set: PieceSet,
    win_condition: WinCondition,
    clock: Option<ClockConfiguration>,
    atomic: Option<Atomic>,
    crazyhouse: Option<Crazyhouse>,
    anti: Option<AntiGame>,
}

impl SpawnGame {
    #[must_use]
    pub fn new(piece_set: PieceSet) -> Self {
        Self {
            players: None,
            piece_set,
            game: Game,
            board: GameBoard::default(),
            turn: CurrentTurn::default(),
            win_condition: WinCondition::default(),
            clock: None,
            atomic: None,
            crazyhouse: None,
            anti: None,
        }
    }

    #[must_use]
    pub fn with_players(mut self, player1: Entity, player2: Entity) -> Self {
        self.players = Some((player1, player2));
        self
    }

    #[must_use]
    pub fn with_board(mut self, board: GameBoard) -> Self {
        self.board = board;
        self
    }

    #[must_use]
    pub fn with_clock(mut self, clock: Option<Clock>) -> Self {
        self.clock = clock.map(|clock| ClockConfiguration { clock });
        self
    }

    #[must_use]
    pub fn atomic(mut self) -> Self {
        self.atomic = Some(Atomic);
        self
    }

    #[must_use]
    pub fn crazyhouse(mut self) -> Self {
        self.crazyhouse = Some(Crazyhouse);
        self
    }

    #[must_use]
    pub fn anti_game(mut self) -> Self {
        self.anti = Some(AntiGame);
        self
    }

    pub fn name(&self) -> Name {
        Name::new(format!("{:?} Game", self.board))
    }
}

impl SpawnGame {
    pub(crate) fn observer(trigger: Trigger<Self>, mut commands: Commands) {
        let spawner = trigger.event();
        let mut builder = commands.spawn((
            spawner.name(),
            spawner.turn,
            spawner.game,
            spawner.board,
            spawner.piece_set.clone(),
            spawner.win_condition.clone(),
            Replicated,
        ));
        if let Some(clock) = &spawner.clock {
            builder.insert(clock.clone());
        }
        if spawner.atomic.is_some() {
            builder.insert(Atomic);
        }
        if spawner.crazyhouse.is_some() {
            builder.insert(Crazyhouse);
        }
        if spawner.anti.is_some() {
            builder.insert(AntiGame);
        }
        let game = builder.id();

        let (player1, player2) = if let Some((player1, player2)) = spawner.players {
            commands.entity(player1).insert(InGame(game));
            commands.entity(player2).insert(InGame(game));
            (player1, player2)
        } else {
            let player1 = commands.spawn((Player, InGame(game))).id();
            let player2 = commands.spawn((Player, InGame(game))).id();
            (player1, player2)
        };
        #[cfg(feature = "log")]
        bevy_log::info!("Spawning a game with new players: {player1}, {player2}");
    }
}
