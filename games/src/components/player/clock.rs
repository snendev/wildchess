use std::time::Duration;

use bevy::{prelude::Component, time::Stopwatch};

#[derive(Clone, Component, Debug, Default)]
pub struct Clock {
    duration: Duration,
    increment: Duration,
    stopwatch: Stopwatch,
}

impl Clock {
    pub fn new(duration: Duration, increment: Duration) -> Self {
        Clock {
            duration,
            increment,
            stopwatch: Stopwatch::new(),
        }
    }

    // N.B. ignores tick if stopwatch is paused
    pub fn tick(&mut self, dt: Duration) {
        self.stopwatch.tick(dt);
    }

    // if there is increment, apply it on pause
    pub fn pause(&mut self) {
        self.stopwatch
            .set_elapsed(self.stopwatch.elapsed() + self.increment);
        self.stopwatch.pause();
    }

    pub fn unpause(&mut self) {
        self.stopwatch.unpause();
    }

    pub fn remaining_time(&self) -> f32 {
        self.duration.as_secs_f32() - self.stopwatch.elapsed_secs()
    }
}
