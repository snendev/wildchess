use serde::{Deserialize, Serialize};

use bevy::prelude::{Commands, Entity, Event, Query, Trigger, With, Without};

use chess::{
    actions::LastAction,
    behavior::PieceBehaviorsBundle,
    board::{Board, OnBoard},
    pieces::{Mutation, PieceIdentity, Position, Royal},
    team::Team,
};

use crate::{
    components::{ActionHistory, CurrentTurn, Game, InGame, Player, Ply},
    Clock,
};

use chess::{actions::Action, pieces::PieceDefinition};

#[derive(Debug)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct PlayTurn {
    pub ply: Ply,
    pub piece: Entity,
    pub board: Entity,
    pub game: Entity,
    pub action: Action,
    pub mutation: Option<PieceDefinition>,
}

impl PlayTurn {
    pub fn action(ply: Ply, piece: Entity, board: Entity, game: Entity, action: Action) -> Self {
        PlayTurn {
            ply,
            piece,
            board,
            game,
            action,
            mutation: None,
        }
    }

    pub fn mutation(
        ply: Ply,
        piece: Entity,
        board: Entity,
        game: Entity,
        action: Action,
        mutated_piece: PieceDefinition,
    ) -> Self {
        PlayTurn {
            ply,
            piece,
            board,
            game,
            action,
            mutation: Some(mutated_piece),
        }
    }
}

impl PlayTurn {
    pub(crate) fn observer(
        trigger: Trigger<PlayTurn>,
        mut commands: Commands,
        mut games: Query<
            (
                &mut CurrentTurn,
                &mut Ply,
                &mut ActionHistory,
                Option<&mut LastAction>,
            ),
            With<Game>,
        >,
        mut boards: Query<Option<&mut LastAction>, (With<Board>, Without<Game>)>,
        mut players: Query<(Entity, &Team, Option<&mut Clock>, &InGame), With<Player>>,
        mut pieces: Query<(Entity, &Team, &mut Position, &OnBoard), With<PieceIdentity>>,
    ) {
        let PlayTurn {
            ply,
            piece,
            board,
            game,
            action,
            mutation,
        } = trigger.event();

        // get the game instance
        let Ok((mut game_turn, mut game_ply, mut game_action_history, game_last_action)) =
            games.get_mut(*game)
        else {
            bevy::log::warn!("Failed to find game {game}");
            return;
        };

        // get the board instance
        let Ok(board_last_action) = boards.get_mut(*board) else {
            bevy::log::warn!("Failed to find board {board}");
            return;
        };

        // is it the correct turn?
        match pieces.get(*piece) {
            Ok((_, team, _, _)) => {
                if *team != game_turn.0 {
                    bevy::log::warn!(
                        "PlayTurn submitted for the wrong team: (piece) {team:?} != (turn) {:?}",
                        game_turn.0
                    );
                    return;
                }
            }
            Err(_error) => {
                bevy::log::warn!("{_error}");
                return;
            }
        }

        // get the piece taking action
        let Ok((_, _, mut piece_square, _)) = pieces.get_mut(*piece) else {
            bevy::log::warn!("Failed to find piece data for {piece}");
            return;
        };

        bevy::log::debug!(
            "Executing {:?}'s turn {ply:?} on board {board}: Moving {piece} {} -> {}",
            game_turn.0,
            action.movement.from,
            action.movement.to
        );

        // execute the primary movement
        piece_square.0 = action.movement.to;

        // execute side effects
        for (side_effect_piece, additional_movement) in action.side_effects.iter() {
            if let Ok((_, _, mut current_square, _)) = pieces.get_mut(*side_effect_piece) {
                current_square.0 = additional_movement.to;
            } else {
                bevy::log::warn!(
                    "Failed to find piece data for {side_effect_piece}: Side effect ignored."
                );
            }
        }

        // execute captures
        for capture_square in action.captures.iter() {
            if let Some(captured_piece) =
                pieces
                    .iter()
                    .find_map(|(capture_entity, _, position, on_board)| {
                        if *position == (*capture_square).into()
                            && capture_entity != *piece
                            && *board == on_board.0
                        {
                            Some(capture_entity)
                        } else {
                            None
                        }
                    })
            {
                bevy::log::debug!("Capturing {captured_piece} on {capture_square}");

                // keep the entity around so that we can maintain its position history
                // and visualize it when viewing old ply
                commands.entity(captured_piece).remove::<Position>();
            }
        }

        // mutate the piece if specified
        if let Some(mutated_piece) = &mutation {
            bevy::log::debug!("Mutating {piece} to {:?}", mutated_piece.identity);

            // remove any existing behaviors and mutation
            commands.entity(*piece).remove::<PieceBehaviorsBundle>();
            commands.entity(*piece).remove::<Mutation>();
            commands.entity(*piece).remove::<PieceIdentity>();
            commands.entity(*piece).remove::<Royal>();

            // TODO: Why is this hack necessary?
            // Without this, the position update is not replicated to clients.
            commands.entity(*piece).remove::<Position>();
            commands.entity(*piece).insert(Position(action.movement.to));

            commands.entity(*piece).insert(mutated_piece.identity);

            // add subsequent mutation if specified
            if let Some(new_mutation) = &mutated_piece.mutation {
                commands.entity(*piece).insert(new_mutation.clone());
            }

            // add Royal if specified
            if mutated_piece.royal.is_some() {
                commands.entity(*piece).insert(Royal);
            }

            // add all specified behaviors
            if let Some(mutation_behavior) = &mutated_piece.behaviors.pattern {
                commands.entity(*piece).insert(mutation_behavior.clone());
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.en_passant {
                commands.entity(*piece).insert(*mutation_behavior);
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.relay {
                commands.entity(*piece).insert(mutation_behavior.clone());
            }
        }

        // update LastAction for the game entity
        if let Some(mut last_move) = game_last_action {
            last_move.0 = action.clone();
        } else {
            commands.entity(*game).insert(LastAction(action.clone()));
        }
        // update LastAction for the board entity
        if let Some(mut last_move) = board_last_action {
            last_move.0 = action.clone();
        } else {
            commands.entity(*board).insert(LastAction(action.clone()));
        }

        // update clocks
        for (_, team, clock, in_game) in players.iter_mut() {
            if *game != in_game.0 {
                continue;
            }
            if game_turn.0 == *team {
                if let Some(mut clock) = clock {
                    clock.pause();
                }
            } else if let Some(mut clock) = clock {
                clock.unpause();
            }
            game_turn.0 = game_turn.0.get_next();
        }

        // TODO: in a future with 4-player, does this lead to bugs?
        if *game_ply == *ply {
            game_action_history.push(*piece, action.clone());
            game_ply.increment();
        } else {
            bevy::log::warn!(
                "Turn ply {:?} does not match current game ply {:?}",
                *ply,
                *game_ply
            );
        }

        // change whose turn it is
        game_turn.0 = game_turn.0.get_next();
    }
}
