use bevy::{
    prelude::{Commands, Component, Entity, In, Query, Reflect, ReflectComponent},
    utils::HashMap,
};

use crate::{
    actions::{Action, Actions},
    board::{Board, Square},
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::Behavior;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct MimicBehavior;

#[derive(Clone, Component, Debug)]
pub struct MimicActionsCache(Actions);

impl From<Actions> for MimicActionsCache {
    fn from(actions: Actions) -> Self {
        MimicActionsCache(actions)
    }
}

impl From<MimicActionsCache> for Actions {
    fn from(cache: MimicActionsCache) -> Self {
        cache.0
    }
}

// Enable performing whatever Pattern was executed in the last turn
impl Behavior for MimicBehavior {
    type ActionsCache = MimicActionsCache;

    fn calculate_actions_system(
        In(last_action): In<Option<Action>>,
        mut commands: Commands,
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Entity,
            Option<&MimicBehavior>,
            Option<&mut MimicActionsCache>,
            &Position,
            &Orientation,
            &Team,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, _, _, position, _, team)| (position.0, *team))
            .collect();

        if let Some(last_action) = last_action {
            for (entity, mimic, cache, position, orientation, team) in piece_query.iter_mut() {
                if mimic.is_some() {
                    let actions =
                        MimicActionsCache::from(Actions::new(last_action.using_pattern.search(
                            &position.0,
                            &orientation,
                            team,
                            board,
                            &pieces,
                            Some(&last_action),
                        )));
                    if let Some(mut cache) = cache {
                        *cache = actions;
                    } else {
                        commands.entity(entity).insert(actions);
                    }
                }
            }
        }
    }
}
