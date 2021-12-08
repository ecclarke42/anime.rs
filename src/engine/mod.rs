use std::sync::{Arc, RwLock};

use generational_arena::{Arena, Index};
use gloo::events::EventListener;
use wasm_bindgen::UnwrapThrowExt;

use crate::{target::Target, util::document_is_hidden};

mod instance;
mod interface;
mod internals;

pub use instance::{Animation, Instance};
pub use interface::Handle;

// TODO: lazy static?
thread_local! {
    pub static GLOBAL: Engine = EngineBuilder::default()
        .init()
        .expect("Failed to initialize global engine");
}

/// Get a handle to the default global engine
pub fn global() -> Engine {
    GLOBAL.with(|e| e.clone())
}

#[derive(Debug, Clone)]
pub struct Engine(internals::EngineRef);

#[derive(Debug, Default)]
pub struct EngineBuilder {
    pub suspend_while_hidden: Option<bool>,
    pub speed: Option<Speed>,
}

impl EngineBuilder {
    fn init(self) -> Result<Engine, EngineError> {
        let inner = internals::new_ref(self);

        // Hold a scoped write guard to set listeners with references to self
        {
            let engine = inner.clone();
            let mut guard = inner.write()?;
            guard.listen_for_visibility_change(engine)
        }

        let engine = Engine(inner);
        engine.run()?;
        Ok(engine)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Engine Async Read Error")]
    ReadError,

    #[error("Engine Async Write Error")]
    WriteError,

    #[error("Failed to call requestAnimationFrame")]
    RafFailure,
}

impl<'a, T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>>> for EngineError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>>) -> Self {
        Self::ReadError
    }
}
impl<'a, T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>> for EngineError {
    fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>) -> Self {
        Self::WriteError
    }
}

// TODO: Move
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Speed {
    Normal,
    Multiplied(f32),
}
impl Speed {
    // pub fn recip(&self) -> f32 {
    //     if let Speed::Multiplied(speed) = self {
    //         speed.recip()
    //     } else {
    //         1f32
    //     }
    // }
}

impl Engine {
    pub fn running(&self) -> bool {
        self.0
            .read()
            .map(|inner| inner.running())
            .unwrap_or_default()
    }

    pub fn speed(&self) -> Speed {
        self.0
            .read()
            .map(|inner| inner.speed)
            .unwrap_or(Speed::Normal)
    }

    // TODO: add Instance instead of
    pub fn add<T: Into<Target>, I: Into<Instance>>(&self, target: T, animation: I) -> Handle {
        // TODO
        todo!()
    }

    /// Start the engine if not already running
    pub fn run(&self) -> Result<(), EngineError> {
        internals::run(self.0.clone())
    }
}
