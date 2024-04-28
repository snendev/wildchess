#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Commands, Component, Entity, In, Query};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

use crate::{
    actions::{Action, Actions},
    behavior::BoardPieceCache,
    board::Board,
    pieces::{Orientation, Position},
    team::Team,
};

use crate::behavior::Behavior;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
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
        board_query: Query<(&Board, &BoardPieceCache)>,
        mut piece_query: Query<(
            Entity,
            Option<&MimicBehavior>,
            Option<&mut MimicActionsCache>,
            &Position,
            &Orientation,
            &Team,
        )>,
    ) {
        let Ok((board, pieces)) = board_query.get_single() else {
            return;
        };

        let Some(last_action) = last_action else {
            return;
        };

        let Some(using_pattern) = &last_action.using_pattern else {
            return;
        };

        for (entity, mimic, cache, position, orientation, team) in piece_query.iter_mut() {
            if mimic.is_some() {
                let actions = MimicActionsCache::from(Actions::new(using_pattern.search(
                    &position.0,
                    orientation,
                    team,
                    board,
                    &pieces.teams,
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
