use bevy::{
    prelude::{Component, In, Query, Reflect, ReflectComponent},
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
        board_query: Query<&Board>,
        mut piece_query: Query<(
            Option<&MimicBehavior>,
            &Position,
            &Orientation,
            &Team,
            &mut MimicActionsCache,
        )>,
    ) {
        let Ok(board) = board_query.get_single() else {
            return;
        };

        let pieces: HashMap<Square, Team> = piece_query
            .iter()
            .map(|(_, position, _, team, _)| (position.0, *team))
            .collect();

        if let Some(last_action) = last_action {
            for (mimic, position, orientation, team, mut cache) in piece_query.iter_mut() {
                if mimic.is_some() {
                    *cache = Actions::new(last_action.using_pattern.search(
                        &position.0,
                        &orientation,
                        team,
                        board,
                        &pieces,
                        Some(&last_action),
                    ))
                    .into();
                }
            }
        }
    }
}
