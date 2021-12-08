//! Public interface for building Animations

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
};

use crate::{
    easings::Easing,
    engine::{Animation, Engine, Handle, Instance},
    properties::{CssProperty, Property, PropertyValue, Unit, Value},
    target::Target,
    Direction, Repeat,
};

use self::property::Parameters;

pub(crate) mod property;

pub trait AnimationCallback: 'static + Debug + Fn(&Animation) {} // TODO: closure type
impl<T> AnimationCallback for T where T: 'static + Debug + Fn(&Animation) {}

// TODO: option<Arc<dyn>> callbacks for Clone?
pub struct Builder {
    engine: Engine,
    target: Arc<Target>,

    pub(crate) properties: Vec<(Property, Parameters)>, // TODO: hashmap? of enum discriminant?

    duration: Option<ParameterValue<u32>>,
    delay: Option<ParameterValue<u32>>,
    end_delay: Option<ParameterValue<u32>>,

    easing: Option<Easing>,
    direction: Option<Direction>,
    repeat: Option<Repeat>,

    on_update: Option<Box<dyn AnimationCallback>>,

    on_begin: Option<Box<dyn AnimationCallback>>,
    on_complete: Option<Box<dyn AnimationCallback>>,

    on_loop_begin: Option<Box<dyn AnimationCallback>>,
    on_loop_complete: Option<Box<dyn AnimationCallback>>,

    on_change: Option<Box<dyn AnimationCallback>>,
    on_change_begin: Option<Box<dyn AnimationCallback>>,
    on_change_complete: Option<Box<dyn AnimationCallback>>,
}

// pub struct Keyframe<T> {
//     props: HashMap<Property, PropertyValue<T>>,
//     // frame params
// }

impl Engine {
    pub fn animate<T: Into<Target>>(&self, target: T) -> Builder {
        Builder::new(self.clone(), target.into())
    }
}

impl Builder {
    fn new(engine: Engine, target: Target) -> Self {
        Self {
            engine,
            target: Arc::new(target),

            properties: Vec::new(),

            duration: None,
            delay: None,
            end_delay: None,

            easing: None,
            direction: None,
            repeat: None,

            on_update: None,
            on_begin: None,
            on_complete: None,
            on_loop_begin: None,
            on_loop_complete: None,
            on_change: None,
            on_change_begin: None,
            on_change_complete: None,
        }
    }

    // TODO
    // pub fn with(&mut self, options: AnimationOptions) -> &mut Self {
    //     self.options = options;
    //     self
    // }

    // pub fn keyframes(&mut self, keyframes: Vec<Keyframe>) -> &mut Self {
    //     for kf in keyframes {
    //         todo!() // TODO: flatten to props
    //     }
    // }

    // pub fn property(&mut self, property: Property, value: PropertyValue) -> &mut Self {
    //     self.properties
    // }

    // TODO: just return an animation
    pub fn finish(self) -> Handle {
        let Builder { engine, target, .. } = self;
        // engine.add(target, options);

        todo!()
    }
}

// TODO: combine with PropertyValue?
enum ParameterValue<T> {
    Value(T),
    Computed(Box<dyn Fn(&Target, usize, usize) -> T>),
}

// use crate::properties::Color;
impl Builder {
    pub fn easing(&mut self, value: Easing) -> &mut Self {
        self.easing = Some(value);
        self
    }

    pub fn duration<T: Into<u32>>(&mut self, value: T) -> &mut Self {
        self.duration = Some(ParameterValue::Value(value.into()));
        self
    }

    pub fn duration_fn<F: 'static + Fn(&Target, usize, usize) -> T, T: Into<u32>>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.duration = Some(ParameterValue::Computed(Box::new(move |el, i, l| {
            f(el, i, l).into()
        })));
        self
    }

    pub fn delay<T: Into<u32>>(&mut self, value: T) -> &mut Self {
        self.delay = Some(ParameterValue::Value(value.into()));
        self
    }

    pub fn delay_fn<F: 'static + Fn(&Target, usize, usize) -> T, T: Into<u32>>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.delay = Some(ParameterValue::Computed(Box::new(move |el, i, l| {
            f(el, i, l).into()
        })));
        self
    }

    pub fn end_delay<T: Into<u32>>(&mut self, value: T) -> &mut Self {
        self.end_delay = Some(ParameterValue::Value(value.into()));
        self
    }

    pub fn end_delay_fn<F: 'static + Fn(&Target, usize, usize) -> T, T: Into<u32>>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.end_delay = Some(ParameterValue::Computed(Box::new(move |el, i, l| {
            f(el, i, l).into()
        })));
        self
    }

    pub fn alternate(&mut self) -> &mut Self {
        self.direction = Some(Direction::Alternate);
        self
    }

    pub fn repeat(&mut self, n_times: u32) -> &mut Self {
        self.repeat = Some(Repeat::Finite {
            current: 0,
            total: n_times,
        });
        self
    }

    pub fn repeat_forever(&mut self) -> &mut Self {
        self.repeat = Some(Repeat::Infinite);
        self
    }
}
