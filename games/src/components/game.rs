use bevy::prelude::{Commands, Component, Deref};

use chess::board::{Rank, Square};
use layouts::PieceSpecification;

use super::Clock;

#[derive(Clone, Copy, Debug, Default, Component)]
pub struct Game;

#[derive(Clone, Copy, Debug, Default, Component)]
pub enum GameBoard {
    #[default]
    Chess,
    // Shogi,    // TODO
    // Checkers, // TODO
}

#[derive(Clone, Component, Deref, Debug, Default)]
pub struct PieceSet(Vec<PieceSpecification>);

impl From<Vec<PieceSpecification>> for PieceSet {
    fn from(pieces: Vec<PieceSpecification>) -> Self {
        Self(pieces)
    }
}

// A game rule specifying that captures result in an "explosion"
// additionally capturing on all squares in the region of the capture.
#[derive(Clone, Debug, Default, Component)]
pub struct Atomic;

// A game rule specifying that players can place captured pieces
// on the board using a turn.
#[derive(Clone, Debug, Default, Component)]
pub struct Crazyhouse;

// A game rule specifying that the typical win condition results in a loss;
// Pieces must capture if they are able to.
#[derive(Clone, Debug, Default, Component)]
pub struct AntiGame;

// The set of win conditions for the board
#[derive(Clone, Debug, Default, Component)]
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

#[derive(Clone, Debug, Default, Component)]
pub struct ClockConfiguration {
    pub clock: Clock,
}

#[derive(Default)]
pub struct GameSpawner {
    game: Game,
    board: GameBoard,
    win_condition: WinCondition,
    piece_set: PieceSet,
    clock: Option<ClockConfiguration>,
    atomic: Option<Atomic>,
    crazyhouse: Option<Crazyhouse>,
    anti: Option<AntiGame>,
}

impl GameSpawner {
    #[must_use]
    pub fn new_game(board: GameBoard, piece_set: PieceSet, win_condition: WinCondition) -> Self {
        Self {
            board,
            piece_set,
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
            .spawn((self.game, self.board, self.piece_set, self.win_condition))
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
