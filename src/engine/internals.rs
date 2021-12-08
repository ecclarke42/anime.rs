use std::sync::{Arc, RwLock};

use generational_arena::{Arena};
use gloo::events::EventListener;
use wasm_bindgen::UnwrapThrowExt;

use super::{Instance};
use crate::{ util::document_is_hidden};

use super::Speed;

pub type EngineRef = Arc<RwLock<Engine>>;

pub fn new_ref(opts: super::EngineBuilder) -> EngineRef {
    Arc::new(RwLock::new(Engine {
        visiblity_listener: None,

        suspend_while_hidden: opts.suspend_while_hidden.unwrap_or(true),
        speed: opts.speed.unwrap_or(Speed::Normal),

        frame: None,
        instances: Arena::new(),
    }))
}

#[derive(Debug)]
pub struct Engine {
    visiblity_listener: Option<EventListener>,

    suspend_while_hidden: bool,
    pub(super) speed: super::Speed,

    /// When running, this is the last value returned by the browser's `requestAnimationFrame` loop
    frame: Option<i32>,

    pub(super) instances: Arena<Instance>,
}

impl Engine {
    pub fn running(&self) -> bool {
        self.frame.is_some()
    }

    pub fn can_run(&self) -> bool {
        !self.suspend_while_hidden || !document_is_hidden()
    }

    pub fn step(&mut self, time: i32) {
        for (_, instance) in self.instances.iter_mut() {
            if instance.active() {
                instance.tick(time)
            }
        }

        // Call next request_animation_frame
    }

    /// Set up the "visibilitychange" listener (with an external Arc to self)
    pub fn listen_for_visibility_change(&mut self, engine: EngineRef) {
        self.visiblity_listener = Some(gloo::events::EventListener::new(
            &crate::util::document(),
            "visibilitychange",
            move |_evt| {
                if let Ok(mut this) = engine.clone().write() {
                    if this.suspend_while_hidden {
                        if crate::util::document_is_hidden() {
                            this.frame
                                .take()
                                .map(|frame| crate::util::window().cancel_animation_frame(frame))
                                .expect_throw("Failed to cancel animation");
                        } else {
                            for (_, instance) in this.instances.iter_mut() {
                                instance.on_document_visibility()
                            }
                            // TODO: scope below?
                            run(engine.clone()); // TODO: error
                        }
                    }
                }
            },
        ))
    }
}

pub(super) fn frame_callback(engine: EngineRef, t: i32) {
    engine
        .clone()
        .write()
        .expect_throw("Failed to call animation frame")
        .step(t);

    let engine_ref = engine.clone();
    if let Ok(frame) =
        crate::util::request_animation_frame(move |t| frame_callback(engine_ref.clone(), t))
    {
        engine
            .write()
            .expect_throw("Failed to call animation frame")
            .frame = Some(frame);
    }
    // Else, handle error?
}

pub(super) fn run(engine: EngineRef) -> Result<(), super::EngineError> {
    let enable = {
        let inner = engine.read()?;
        !inner.running() && inner.can_run()
    };
    if enable {
        let engine_ref = engine.clone();
        engine
            .write()
            .map_err(|_| super::EngineError::WriteError)?
            .frame = Some(
            crate::util::request_animation_frame(move |t| frame_callback(engine_ref.clone(), t))
                .map_err(|_| super::EngineError::RafFailure)?,
        );
    }

    Ok(())
}
