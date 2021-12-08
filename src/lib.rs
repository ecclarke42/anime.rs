// TODO: wasm-bindgen to make available from js?

mod builder;
pub mod css;
mod easings;
mod engine;
mod path;
mod properties;
mod target;
mod util;

pub use engine::Engine;

/// Get a handle to the default global engine
pub fn engine() -> Engine {
    engine::global()
}

pub fn animate<T: Into<target::Target>>(target: T) -> builder::Builder {
    engine::global().animate(target)
}

pub mod prelude {

    pub use palette;

    pub use super::animate;
    pub use crate::builder::property::Property;
    pub use crate::easings::{helpers::*, Direction as EasingDirection, Easing};
    pub use crate::properties::{Unit, Value, ValueOp};
}

// TODO: Mod

pub enum Stagger<T> {
    Value(T, Option<StaggerProps>),

    Range(T, T),
}
pub struct StaggerProps {
    start: Option<u32>,
    from: Option<StaggerFrom>,
    direction: Option<StaggerDirection>,
    // easing: Option<Easing>,
    grid: Option<(u32, u32)>,
    axis: Option<StaggerGridAxis>,
}

pub enum StaggerFrom {
    First, // default
    Last,
    Center,
    Index(u32),
}

pub enum StaggerDirection {
    Normal, // default
    Reverse,
}

pub enum StaggerGridAxis {
    X,
    Y,
}

// TODO: move below to animation::parameters

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Normal,
    Reverse,
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repeat {
    Finite { current: u32, total: u32 },
    Infinite,
}
impl Repeat {
    fn remaining(&self) -> Option<u32> {
        if let Repeat::Finite { current, total } = self {
            Some(total - current)
        } else {
            None
        }
    }
    fn add(&mut self, repetitions: u32) {
        if let Repeat::Finite { current, total } = self {
            *total += repetitions
        }
    }
}
