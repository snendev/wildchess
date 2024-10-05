use serde::{Deserialize, Serialize};

use bevy::prelude::{Commands, Component, Entity, Event, Name, Reflect, Trigger};

use bevy_replicon::prelude::Replicated;

use chess::{
    behavior::{BoardPieceCache, BoardThreatsCache},
    board::{Board, OnBoard, Rank, Square},
    pieces::{PieceBundle, Position, Royal},
    team::Team,
};
use layouts::PieceSpecification;

use crate::{
    components::{ActionHistory, History, Ply},
    Clock,
};

use super::{InGame, Player};

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
pub struct Game;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
pub enum GameBoard {
    #[default]
    Chess,
    // Shogi,    // TODO
    // Checkers, // TODO
}

#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct PieceSet(pub Vec<PieceSpecification>);

impl From<Vec<PieceSpecification>> for PieceSet {
    fn from(pieces: Vec<PieceSpecification>) -> Self {
        Self(pieces)
    }
}

// A game rule specifying that captures result in an "explosion"
// additionally capturing on all squares in the region of the capture.
#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Atomic;

// A game rule specifying that players can place captured pieces
// on the board using a turn.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
pub struct Crazyhouse;

// A game rule specifying that the typical win condition results in a loss;
// Pieces must capture if they are able to.
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
pub struct AntiGame;

// The set of win conditions for the board
#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
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
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct ClockConfiguration {
    pub clock: Clock,
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Deserialize, Serialize)]
#[derive(Component, Reflect)]
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

    pub fn clock(&self) -> Option<&ClockConfiguration> {
        self.clock.as_ref()
    }
}

impl SpawnGame {
    pub(crate) fn observer(trigger: Trigger<Self>, mut commands: Commands) {
        let spawner = trigger.event();

        // first spawn the game entity
        // this has all the information about the game instance's configuration
        let mut builder = commands.spawn((
            spawner.name(),
            spawner.turn,
            spawner.game,
            spawner.board,
            spawner.piece_set.clone(),
            spawner.win_condition.clone(),
            Replicated,
            Ply::default(),
            ActionHistory::default(),
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

        // next spawn a board entity that will track board state
        let board_data = match spawner.board {
            GameBoard::Chess => Board::chess_board(),
        };
        let board = commands
            .spawn((
                board_data,
                InGame(game),
                Name::new(format!("Board (Game {:?})", game)),
                BoardPieceCache::default(),
                BoardThreatsCache::default(),
                Replicated,
            ))
            .id();

        // next find or spawn our players and associate them with the game instance and board
        let (player1, player2) = if let Some((player1, player2)) = spawner.players {
            (player1, player2)
        } else {
            (
                commands.spawn(Name::new("Player")).id(),
                commands.spawn(Name::new("Player")).id(),
            )
        };

        for (player, team) in [(player1, Team::White), (player2, Team::Black)] {
            commands
                .entity(player)
                .insert((
                    Player,
                    InGame(game),
                    OnBoard(board),
                    team,
                    team.orientation(),
                ))
                .try_insert(Replicated);
            if let Some(clock) = spawner.clock() {
                commands.entity(player).insert(clock.clock.clone());
            }
        }

        // finally, spawn all game pieces
        for team in [Team::White, Team::Black].into_iter() {
            for PieceSpecification {
                piece,
                start_square,
            } in spawner.piece_set.0.iter()
            {
                let start_square = start_square.reorient(team.orientation(), &board_data);
                let name = Name::new(format!("{:?} {}-{:?}", team, start_square, piece.identity));
                bevy::log::debug!("...spawning piece: {}", name);

                let mut piece_builder = commands.spawn((
                    name,
                    piece.identity,
                    PieceBundle::new(start_square.into(), team),
                    InGame(game),
                    OnBoard(board),
                    History::<Position>::default(),
                    Replicated,
                ));

                if piece.royal.is_some() {
                    piece_builder.insert(Royal);
                }
                if let Some(mutation) = &piece.mutation {
                    piece_builder.insert(mutation.clone());
                }
                if let Some(behavior) = &piece.behaviors.pattern {
                    piece_builder.insert(behavior.clone());
                }
                if let Some(behavior) = &piece.behaviors.relay {
                    piece_builder.insert(behavior.clone());
                }
                if let Some(behavior) = piece.behaviors.en_passant {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = piece.behaviors.castling {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = piece.behaviors.castling_target {
                    piece_builder.insert(behavior);
                }
            }
        }

        bevy::log::info!("Spawned game {game} with players {player1}, {player2} on board {board}");
    }
}
