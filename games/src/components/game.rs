#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::{Commands, Component};

use chess::board::{Rank, Square};

use super::Clock;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Game;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum GameBoard {
    Chess,
    #[default]
    WildChess,
    SuperRelayChess,
    KnightRelayChess,
    // Shogi,    // TODO
    // Checkers, // TODO
}

// A game rule specifying that captures result in an "explosion"
// additionally capturing on all squares in the region of the capture.
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Atomic;

// A game rule specifying that players can place captured pieces
// on the board using a turn.
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Crazyhouse;

// A game rule specifying that the typical win condition results in a loss;
// Pieces must capture if they are able to.
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AntiGame;

// The set of win conditions for the board
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum WinCondition {
    // The game is won once all enemy Royal pieces are captured.
    #[default]
    RoyalCaptureAll,
    // The game is won once a single enemy Royal piece is captured.
    RoyalCapture,
    // The game is won once a Royal piece reaches a specific Rank.
    // (The Rank is local to the player's Orientation.)
    RaceToRank(Rank),
    //The game is won once a Royal piece reaches any of the given Squares.
    RaceToRegion(Vec<Square>),
}

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ClockConfiguration {
    pub clock: Clock,
}

// TODO: revisit this API
// perhaps use the blueprints lib
#[derive(Default)]
pub struct GameSpawner {
    pub game: Game,
    pub board: GameBoard,
    pub win_condition: WinCondition,
    pub clock: Option<ClockConfiguration>,
    pub atomic: Option<Atomic>,
    pub crazyhouse: Option<Crazyhouse>,
    pub anti: Option<AntiGame>,
}

impl GameSpawner {
    #[must_use]
    pub fn new_game(board: GameBoard, win_condition: WinCondition) -> Self {
        Self {
            board,
            win_condition,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_clock(mut self, clock: Clock) -> Self {
        self.clock = Some(ClockConfiguration { clock });
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

    pub fn spawn(self, commands: &mut Commands) {
        let entity = commands
            .spawn((self.game, self.board, self.win_condition))
            .id();
        let mut builder = commands.entity(entity);
        if let Some(clock) = self.clock {
            builder.insert(clock);
        }
        if self.atomic.is_some() {
            builder.insert(Atomic);
        }
        if self.crazyhouse.is_some() {
            builder.insert(Crazyhouse);
        }
        if self.anti.is_some() {
            builder.insert(AntiGame);
        }
    }
}
