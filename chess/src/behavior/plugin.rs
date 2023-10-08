use std::marker::PhantomData;

use bevy::prelude::{
    apply_deferred, App, IntoSystem, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PostUpdate,
    Query, SystemSet,
};

use crate::{
    actions::{Action, Actions},
    behavior::{Behavior, EnPassantBehavior, MimicBehavior, PatternBehavior, RelayBehavior},
};

// N.B. Use this to configure run conditions so that actions are not calculated every frame
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct BehaviorsSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
enum BehaviorsInnerSet {
    PrepareFrame,
    PopulateCache,
    PopulateActions,
}

pub struct BehaviorsPlugin<System, Params>
where
    System: IntoSystem<(), Option<Action>, Params>,
{
    on_action: System,
    marker: PhantomData<Params>,
}

impl<System, Params> BehaviorsPlugin<System, Params>
where
    System: IntoSystem<(), Option<Action>, Params>,
{
    pub fn from_input_system(input_system: System) -> Self {
        Self {
            on_action: input_system,
            marker: PhantomData::<Params>,
        }
    }
}

fn clear_actions(mut piece_query: Query<&mut Actions>) {
    for mut actions in piece_query.iter_mut() {
        actions.clear();
    }
}

impl<System, Params> Plugin for BehaviorsPlugin<System, Params>
where
    System: IntoSystem<(), Option<Action>, Params> + Clone + Send + Sync + 'static,
    Params: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                BehaviorsInnerSet::PrepareFrame,
                BehaviorsInnerSet::PopulateCache,
                BehaviorsInnerSet::PopulateActions,
            )
                .chain()
                .in_set(BehaviorsSet),
        )
        .add_systems(
            PostUpdate,
            (
                clear_actions.in_set(BehaviorsInnerSet::PrepareFrame),
                (
                    self.on_action
                        .clone()
                        .pipe(PatternBehavior::calculate_actions_system),
                    self.on_action
                        .clone()
                        .pipe(EnPassantBehavior::calculate_actions_system),
                    self.on_action
                        .clone()
                        .pipe(MimicBehavior::calculate_actions_system),
                    self.on_action
                        .clone()
                        .pipe(RelayBehavior::calculate_actions_system),
                )
                    .in_set(BehaviorsInnerSet::PopulateCache),
                apply_deferred,
                (
                    PatternBehavior::take_actions_system,
                    EnPassantBehavior::take_actions_system,
                    MimicBehavior::take_actions_system,
                    RelayBehavior::take_actions_system,
                )
                    .in_set(BehaviorsInnerSet::PopulateActions),
            )
                .chain(),
        );
    }
}
