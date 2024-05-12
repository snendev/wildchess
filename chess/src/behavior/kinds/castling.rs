use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

use bevy_ecs::prelude::{Commands, Component, DetectChanges, Entity, Query, Ref, With};

#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "log")]
use bevy_log::warn;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{
    actions::{Action, Actions, Movement},
    behavior::{BoardPieceCache, BoardThreatsCache},
    board::{File, Rank, Square},
    pieces::{Orientation, Position},
    team::Team,
};

pub(crate) fn disable_on_move<T: Component>(
    mut commands: Commands,
    moved_piece_query: Query<(Entity, Ref<Position>, Ref<Orientation>), With<T>>,
) {
    for (piece, position_ref, orientation_ref) in moved_piece_query.iter() {
        if (position_ref.is_changed() && !position_ref.is_added())
            || (orientation_ref.is_changed() && !orientation_ref.is_added())
        {
            commands.entity(piece).remove::<T>();
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct CastlingTarget;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct CastlingBehavior;

// Enable performing whatever Pattern was executed in the last turn
impl CastlingBehavior {
    pub(crate) fn calculate_actions_system(
        board_query: Query<(&BoardPieceCache, &BoardThreatsCache)>,
        mut castler_query: Query<
            (&Position, &Team, &Orientation, &mut Actions),
            With<CastlingBehavior>,
        >,
        target_query: Query<(Entity, &Position, &Team, &Orientation), With<CastlingTarget>>,
    ) {
        let Ok((pieces, threats)) = board_query.get_single() else {
            return;
        };

        for (Position(position), team, orientation, mut actions) in castler_query.iter_mut() {
            for (target_entity, Position(target), target_team, target_orientation) in
                target_query.iter()
            {
                if team != target_team {
                    continue;
                }
                let (landing_square, is_horizontal, is_position_gt_target) = match (
                    position.file.0.cmp(&target.file.0),
                    position.rank.0.cmp(&target.rank.0),
                ) {
                    (Ordering::Less, Ordering::Equal) => {
                        (Square::new(File::G, position.rank), true, false)
                    }
                    (Ordering::Greater, Ordering::Equal) => {
                        (Square::new(File::C, position.rank), true, true)
                    }
                    (Ordering::Equal, Ordering::Less) => {
                        (Square::new(position.file, Rank::SIX), false, false)
                    }
                    (Ordering::Equal, Ordering::Greater) => {
                        (Square::new(position.file, Rank::TWO), false, true)
                    }
                    _ => {
                        #[cfg(feature = "log")]
                        warn!("Unexpected castling alignment detected. Castler square: {}, Target square: {}", position, target);
                        continue;
                    }
                };

                // the piece could still be on either side of the target, so we have to check
                let scanned_squares: Vec<_> = if is_horizontal {
                    match position.file.cmp(&landing_square.file) {
                        Ordering::Less => ((position.file.0 + 1)..=landing_square.file.0)
                            .map(|file| Square::new(File(file), landing_square.rank))
                            .collect(),
                        Ordering::Greater => (landing_square.file.0
                            ..=position.file.0.saturating_sub(1))
                            .rev()
                            .map(|file| Square::new(File(file), landing_square.rank))
                            .collect(),
                        Ordering::Equal => {
                            vec![]
                        }
                    }
                } else {
                    match position.rank.cmp(&landing_square.rank) {
                        Ordering::Less => ((position.rank.0 + 1)..=landing_square.rank.0)
                            .map(|rank: u16| Square::new(landing_square.file, Rank(rank)))
                            .collect(),
                        Ordering::Greater => (landing_square.rank.0
                            ..=position.rank.0.saturating_sub(1))
                            .rev()
                            .map(|rank| Square::new(landing_square.file, Rank(rank)))
                            .collect(),
                        Ordering::Equal => {
                            vec![]
                        }
                    }
                };

                // the castle target should appear on the "other" side of the castler
                let target_landing_square = if is_horizontal {
                    Square::new(
                        if is_position_gt_target {
                            File(landing_square.file.0 + 1)
                        } else {
                            File(landing_square.file.0 - 1)
                        },
                        landing_square.rank,
                    )
                } else {
                    Square::new(
                        landing_square.file,
                        if is_position_gt_target {
                            Rank(landing_square.rank.0 + 1)
                        } else {
                            Rank(landing_square.rank.0 - 1)
                        },
                    )
                };

                let is_in_check = threats.is_threatened(*position, *team);
                let is_forbidden_movement = scanned_squares.iter().any(|scan|
                        // scanned square is check
                        threats.is_threatened(*scan, *team)
                        ||
                        // movement collides with piece (except the rook)
                        *scan != *target && pieces.teams.contains_key(scan));
                if let Some(square) = scanned_squares.iter().find(|scan|
                            // scanned square is check
                            threats.is_threatened(**scan, *team)
                            ||
                            // movement collides with piece (except the rook)
                            **scan != *target && pieces.teams.contains_key(*scan))
                {
                    eprintln!("{} targeted", square,);
                }
                let collides_rook = target_landing_square != *target
                    && pieces.teams.contains_key(&target_landing_square);

                if !is_in_check && !is_forbidden_movement && !collides_rook {
                    actions.0.insert(
                        *target,
                        Action {
                            movement: Movement {
                                from: *position,
                                to: landing_square,
                                orientation: *orientation,
                            },
                            side_effects: vec![(
                                target_entity,
                                Movement {
                                    from: *target,
                                    to: target_landing_square,
                                    orientation: *target_orientation,
                                },
                            )],
                            scanned_squares,
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use bevy_app::prelude::{App, PostUpdate};
    use bevy_ecs::prelude::{Entity, IntoSystemConfigs, World};
    use bevy_utils::HashSet;

    use crate::{
        actions::{Action, Actions},
        behavior::{BoardPieceCache, BoardThreatsCache},
        board::{Board, OnBoard, Square},
        pieces::PieceBundle,
        team::Team,
    };

    use super::{CastlingBehavior, CastlingTarget};

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_systems(
            PostUpdate,
            (
                BoardPieceCache::track_pieces,
                BoardThreatsCache::track_pieces,
                CastlingBehavior::calculate_actions_system,
            )
                .chain(),
        );
        app
    }

    fn spawn_board(world: &mut World) -> Entity {
        world
            .spawn((
                Board::chess_board(),
                BoardPieceCache::default(),
                BoardThreatsCache::default(),
            ))
            .id()
    }

    #[test]
    fn test_castling() -> Result<()> {
        let mut app = setup_app();
        spawn_board(&mut app.world);

        // prep a king, rook, and board
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e1")?.into(), Team::White),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("h1")?.into(), Team::White),
            CastlingTarget,
        ));
        app.update();

        // check that the king and rook end up on h1 and g1, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("h1")?).unwrap().clone();
        assert_eq!(castle_action.movement.to, Square::try_from("g1")?);
        assert_eq!(
            castle_action.side_effects.first().unwrap().1.to,
            Square::try_from("f1")?
        );

        Ok(())
    }

    #[test]
    fn test_long_castle() -> Result<()> {
        let mut app = setup_app();
        spawn_board(&mut app.world);

        // prep a king, rook, and board
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e8")?.into(), Team::White),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("a8")?.into(), Team::White),
            CastlingTarget,
        ));
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("a8")?).unwrap().clone();
        assert_eq!(castle_action.movement.to, Square::try_from("c8")?);
        assert_eq!(
            castle_action.side_effects.first().unwrap().1.to,
            Square::try_from("d8")?
        );

        Ok(())
    }

    #[test]
    fn test_960_castle() -> Result<()> {
        let mut app = setup_app();
        spawn_board(&mut app.world);

        // prep a king, rook, and board
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("b1")?.into(), Team::White),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("a1")?.into(), Team::White),
            CastlingTarget,
        ));
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("a1")?).unwrap().clone();
        assert_eq!(castle_action.movement.to, Square::try_from("c1")?);
        assert_eq!(
            castle_action.side_effects.first().unwrap().1.to,
            Square::try_from("d1")?
        );

        Ok(())
    }

    #[test]
    fn test_piece_in_middle() -> Result<()> {
        let mut app = setup_app();
        let board = spawn_board(&mut app.world);

        // prep a king, rook
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e8")?.into(), Team::White),
                OnBoard(board),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("h8")?.into(), Team::White),
            OnBoard(board),
            CastlingTarget,
        ));
        // and another piece on the king's path
        app.world.spawn((
            PieceBundle::new(Square::try_from("f8")?.into(), Team::Black),
            OnBoard(board),
        ));

        // run a tick
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("h8")?);
        assert_eq!(castle_action, None);

        Ok(())
    }

    #[test]
    fn test_king_in_check() -> Result<()> {
        let mut app = setup_app();
        let board = spawn_board(&mut app.world);

        // prep a king, rook
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e1")?.into(), Team::White),
                OnBoard(board),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("h1")?.into(), Team::White),
            OnBoard(board),
            CastlingTarget,
        ));

        // and a piece attacking the king's square
        let mut threat_bundle = PieceBundle::new(Square::try_from("e3")?.into(), Team::Black);
        let mut threats = HashSet::new();
        threats.insert(Square::try_from("e1")?);
        threat_bundle.actions.0.insert(
            Square::try_from("e1")?,
            Action {
                captures: threats.clone(),
                threats,
                ..Default::default()
            },
        );
        app.world.spawn((threat_bundle, OnBoard(board)));

        // run a tick
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("h8")?);
        assert_eq!(castle_action, None);

        Ok(())
    }

    #[test]
    fn test_threatened_movement() -> Result<()> {
        let mut app = setup_app();
        let board = spawn_board(&mut app.world);

        // prep a king, rook
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e8")?.into(), Team::White),
                OnBoard(board),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("h8")?.into(), Team::White),
            OnBoard(board),
            CastlingTarget,
        ));

        // and a piece attacking along the king's path
        let mut threat_bundle = PieceBundle::new(Square::try_from("f8")?.into(), Team::Black);
        let mut threats = HashSet::new();
        threats.insert(Square::try_from("e8")?);
        threat_bundle.actions.0.insert(
            Square::default(),
            Action {
                captures: threats.clone(),
                threats,
                ..Default::default()
            },
        );
        app.world.spawn((threat_bundle, OnBoard(board)));

        // run a tick
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("h8")?);
        assert_eq!(castle_action, None);

        Ok(())
    }

    #[test]
    fn test_threatened_target_square() -> Result<()> {
        let mut app = setup_app();
        let board = spawn_board(&mut app.world);

        // prep a king, rook
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("e1")?.into(), Team::White),
                OnBoard(board),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("h1")?.into(), Team::White),
            OnBoard(board),
            CastlingTarget,
        ));

        // and a piece attacking the square the king will land on
        let mut threat_bundle = PieceBundle::new(Square::try_from("f1")?.into(), Team::Black);

        let mut threats = HashSet::new();
        threats.insert(Square::try_from("g1")?);
        threat_bundle.actions.0.insert(
            Square::default(),
            Action {
                captures: threats.clone(),
                threats,
                ..Default::default()
            },
        );
        app.world.spawn((threat_bundle, OnBoard(board)));

        // run a tick
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("h8")?);
        assert_eq!(castle_action, None);

        Ok(())
    }

    #[test]
    fn test_rook_collides() -> Result<()> {
        let mut app = setup_app();
        let board = spawn_board(&mut app.world);

        // prep a king, rook
        let king = app
            .world
            .spawn((
                PieceBundle::new(Square::try_from("b1")?.into(), Team::White),
                OnBoard(board),
                CastlingBehavior,
            ))
            .id();
        app.world.spawn((
            PieceBundle::new(Square::try_from("a1")?.into(), Team::White),
            OnBoard(board),
            CastlingTarget,
        ));
        // and a piece on the square the rook would travel to
        app.world.spawn((
            PieceBundle::new(Square::try_from("d1")?.into(), Team::White),
            OnBoard(board),
        ));

        // run a tick
        app.update();

        // check that the king and rook end up on c8 and d8, respectively
        let actions = app.world.entity(king).get::<Actions>().unwrap();
        let castle_action = actions.0.get(&Square::try_from("a1")?);
        assert_eq!(castle_action, None);

        Ok(())
    }
}
