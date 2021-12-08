use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

use crate::{
    easings::Easing,
    engine::Speed,
    Repeat,
    properties::{Property, Value},
    target::Target,
    Direction, Engine,
};

#[derive(Debug)]
pub struct Instance {
    // id: u32, // TODO: not necessary?
    engine: Engine, // hold reference to parent engine

    /// Set to true to disable event callbacks (mainly for timelines)
    passthrough: bool, // TODO: not necessary?

    autoplay: bool, // TODO: necessary? Just start paused?

    began: bool,
    loop_began: bool,
    change_began: bool,
    paused: bool,
    completed: bool,
    change_completed: bool,

    repeat: Repeat,

    start_time: f32,
    current_time: f32,
    last_time: f32,
    now: f32,
    progress: u32,

    children: Vec<Instance>,
    animations: Vec<Animation>,

    // Timings // TODO: getInstanceTimings
    duration: f32,
    delay: f32,
    end_delay: f32,

    // resolve
    direction: Direction,
    reversed: bool,
    reverse_playback: bool,
}

// For a single target
struct Animatable {
    // From targets
    target: Target,
    total: u32, // length of parent?
    transforms: (), // {
                // list: Vec<getElementTransforms>
                // }
}

// For each property on an "animatable"
#[derive(Debug, PartialEq)]
pub struct Animation {
    target: Target,         //?
    property: Property,     // TODO
    tweens: Vec<Tween<()>>, // tweens // TODO: value type, based on property?

    // See getAnimations
    duration: f32, // Default: 1000
    delay: f32,
    end_delay: f32,
    easing: Easing,
    round: u32,
}

impl Animation {
    // fn duration() -> Option<f32> {

    // }
}

#[derive(Debug, PartialEq)]
struct Tween<T: Debug> {
    from: Value<T>,
    to: Value<T>,

    start: f32,
    end: f32,
    easing: Easing,
    // is_path, is_path_target_inside_svg, is_color
}

impl Instance {
    pub fn active(&self) -> bool {
        true // TODO
    }

    pub fn tick(&mut self, time: i32) {}

    pub fn add(&mut self, child: Instance) {
        todo!() // TODO: timeline specific
    }

    /*

    Public Methods

    */
    pub fn play(&mut self) {
        if !self.paused {
            return;
        }
        if self.completed {
            self.reset()
        }
        self.paused = false;

        // TODO: add to active instances?
        self.reset_time();
        let _ = self.engine.run(); // TODO: error
    }

    pub fn pause(&mut self) {
        self.paused = true;
        self.reset_time();
    }

    pub fn reverse(&mut self) {
        self.direction.reverse();
        self.reversed = !self.reversed;
        // TODO: reverse all children
        self.completed = !self.reversed;
        self.reset_time();
    }

    pub fn restart(&mut self) {
        self.reset();
        self.play();
    }

    // TODO: remove

    pub fn reset(&mut self) {
        self.passthrough = false;
        self.current_time = 0f32;
        self.progress = 0;
        self.paused = true;
        self.began = false;
        self.loop_began = false;
        self.change_began = false;
        self.completed = false;
        self.change_completed = false;
        self.reverse_playback = false;
        self.reversed = matches!(self.direction, Direction::Reverse);

        // self.remaining = instance.loop;

        self.children.iter_mut().for_each(Self::reset);

        if (self.reversed && matches!(self.repeat, Repeat::Finite { .. }))
            || (matches!(self.direction, Direction::Alternate)
                && matches!(self.repeat, Repeat::Finite { total: 1, .. }))
        {
            self.repeat.add(1)
        }

        // TODO: setAnimationsProgress
        self.set_animations_progress(if self.reversed { self.duration } else { 0.0 });
    }
    // TODO: pub?
    fn reset_time(&mut self) {
        self.start_time = 0.0;
        self.last_time = {
            let mut time = if self.reversed {
                (self.duration as f32) - self.current_time
            } else {
                self.current_time
            };
            if let Speed::Multiplied(speed) = self.engine.speed() {
                time /= speed;
            }
            time
        }
    }

    // TODO: multiple prop/value
    pub fn set(target: Target, property: &str, value: &str) {} // Helper -> static? or just function?

    /*

    Private Methods

    */

    pub(crate) fn on_document_visibility(&mut self) {
        self.reset_time()
    }

    fn get_time(&self, engine_time: f32) -> f32 {
        // TODO
        todo!()

        // TODO: EngineTime and InstanceTime wrappers?
    }

    // TODO: for perf, only update animation values here,
    // do the dom interaction elsewhere? so we can let go of the write
    // lock as soon as possible
    fn set_animations_progress(&mut self, instance_time: f32) {
        for animation in self.animations.iter_mut() {
            let fractional_time = todo!();
            // TODO
            // animation.update_
        }
    }

    fn set_instance_progress(&mut self, engine_time: f32) {
        let instance_time = self.get_time(engine_time);
    }
}

impl Direction {
    pub fn reverse(&mut self) {
        use Direction::*;
        *self = match *self {
            Normal => Reverse,
            Reverse => Normal,
            Alternate => Alternate,
        }
    }
}

// TODO: impl future for Animation?
