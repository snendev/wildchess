use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;
use bevy_time::Stopwatch;

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
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

    pub fn remaining_seconds(&self) -> u64 {
        self.duration
            .as_secs()
            .saturating_sub(self.stopwatch.elapsed().as_secs())
    }
}
