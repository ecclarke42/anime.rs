use crate::{
    easings::Easing,
    properties::{PropertyValue, Value},
};

// use super::Parameters;

#[derive(Debug)]
pub struct Property<T> {
    pub(crate) value: PropertyValue<T>,
    pub(crate) params: Parameters,
}

#[derive(Debug, Default)]
pub struct Parameters {
    pub duration: Option<u32>, // default 1000
    pub delay: Option<u32>,
    pub end_delay: Option<u32>,
    pub easing: Option<Easing>,
    pub round: Option<u32>,
}

impl<T> Property<T> {
    pub fn value<V: Into<Value<T>>>(value: V) -> Self {
        Self {
            value: PropertyValue::Exact(value.into()),
            params: Parameters::default(),
        }
    }

    pub fn range<V: Into<Value<T>>>(from: V, to: V) -> Self {
        Self {
            value: PropertyValue::Range(from.into(), to.into()),
            params: Parameters::default(),
        }
    }

    pub fn duration(mut self, value: u32) -> Self {
        self.params.duration = Some(value);
        self
    }
    pub fn delay(mut self, value: u32) -> Self {
        self.params.delay = Some(value);
        self
    }
    pub fn end_delay(mut self, value: u32) -> Self {
        self.params.end_delay = Some(value);
        self
    }
    pub fn easing(mut self, value: Easing) -> Self {
        self.params.easing = Some(value);
        self
    }
    pub fn round(mut self, value: u32) -> Self {
        self.params.round = Some(value);
        self
    }
}

// impl<T> From<Property<T>> for Value<T> {
//     fn from(p: Property<T>) -> Self {
//         p.value
//     }
// }

impl<T: Into<Value<U>>, U> From<T> for Property<U> {
    fn from(t: T) -> Self {
        Self {
            value: PropertyValue::Exact(t.into()),
            params: Parameters::default(),
        }
    }
}
