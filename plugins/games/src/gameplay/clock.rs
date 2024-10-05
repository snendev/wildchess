use chess::team::Team;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy::prelude::{
    App, Commands, Component, Entity, IntoSystemConfigs, Plugin, Query, Reflect, Res, SystemSet,
    Time, Update,
};
use bevy::time::Stopwatch;

use bevy_replicon::prelude::AppRuleExt;

use crate::components::{GameOver, InGame, IsActiveGame};

#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Clock {
    duration: Duration,
    increment: Duration,
    stopwatch: Stopwatch,
}

impl Clock {
    pub fn new(duration: Duration, increment: Duration) -> Self {
        let mut stopwatch = Stopwatch::new();
        stopwatch.pause();
        Clock {
            duration,
            increment,
            stopwatch,
        }
    }

    // N.B. ignores tick if stopwatch is paused
    pub fn tick(&mut self, dt: Duration) {
        self.stopwatch.tick(dt);
    }

    // if there is increment, apply it on pause
    pub fn pause(&mut self) {
        self.duration += self.increment;
        self.stopwatch.pause();
    }

    pub fn unpause(&mut self) {
        self.stopwatch.unpause();
    }

    pub fn remaining_time(&self) -> Duration {
        self.duration.saturating_sub(self.stopwatch.elapsed())
    }

    pub fn is_flagged(&self) -> bool {
        self.remaining_time().is_zero()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct ClockSystems;

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::tick.in_set(ClockSystems));
        app.replicate::<Clock>();
        app.register_type::<Clock>();
    }
}

impl ClockPlugin {
    // ticks clocks for all active games
    fn tick(
        mut commands: Commands,
        mut clocks: Query<(&mut Clock, &Team, &InGame)>,
        games: Query<Entity, IsActiveGame>,
        time: Res<Time>,
    ) {
        for (mut clock, team, in_game) in clocks.iter_mut() {
            let Ok(game) = games.get(in_game.0) else {
                continue;
            };
            clock.tick(time.delta());
            if clock.is_flagged() {
                commands.entity(game).insert(GameOver::new(match team {
                    Team::White => Team::Black,
                    Team::Black => Team::White,
                }));
            }
        }
    }
}
