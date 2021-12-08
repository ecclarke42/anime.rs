use std::fmt::{Debug, Display};

use lazy_static::lazy_static;
use regex::Regex;

use crate::target::Target;

lazy_static! {
    static ref UNIT_REGEX: Regex = Regex::new(
        r"/[+-]?\d*\.?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?(%|px|pt|em|rem|in|cm|mm|ex|ch|pc|vw|vh|vmin|vmax|deg|rad|turn)?$/"
    ).expect("Failed to compile UNIT_REGEX");
}

#[derive(Debug, PartialEq, Clone)]
pub struct Value<T>(T, Option<Unit>); // TODO: just for input, internal repr no option?

impl<T: PartialOrd> PartialOrd for Value<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.1, other.1) {
            (Some(self_unit), Some(other_unit)) if self_unit == other_unit => {
                self.0.partial_cmp(&other.0)
            }
            (None, None) => self.0.partial_cmp(&other.0),

            _ => None,
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(t: T) -> Self {
        Value(t, None)
    }
}

impl<T> From<(T, Unit)> for Value<T> {
    fn from((t, u): (T, Unit)) -> Self {
        Value(t, Some(u))
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyValue<T> {
    // TODO: Remove?
    Exact(Value<T>),
    Relative(ValueOp<T>),
    Range(Value<T>, Value<T>),

    // /// Computed based on the target's position in the array of animated elements
    // /// Arguments are: the target, it's index, and the number of elements
    // Computed(Box<dyn Fn(&Target, usize, usize) -> Value<T>>),
    Keyframes(Vec<PropertyKeyframe<T>>),
}

impl<T> From<T> for PropertyValue<T> {
    fn from(t: T) -> Self {
        PropertyValue::Exact(Value(t, None))
    }
}
impl<T> From<(T, Unit)> for PropertyValue<T> {
    fn from((t, u): (T, Unit)) -> Self {
        PropertyValue::Exact(Value(t, Some(u)))
    }
}

// impl<T, U: Into<Value<T>>> From<std::ops::RangeInclusive<U>> for PropertyValue<T> {
//     fn from(r: std::ops::RangeInclusive<U>) -> Self {
//         PropertyValue::Range(r.start().into(), r.end().into())
//     }
// }
// impl<T> From<(O) // TODO: relative?
// impl<T: Clone> From<std::ops::RangeInclusive<T>> for PropertyValue<T> {
//     fn from(r: std::ops::RangeInclusive<T>) -> Self {
//         PropertyValue::Range(Value(r.start().clone(), None), Value(r.end().clone(), None))
//     }
// }
// impl<T: Clone + PartialOrd> From<std::ops::RangeInclusive<Value<T>>> for PropertyValue<T> {
//     fn from(r: std::ops::RangeInclusive<Value<T>>) -> Self {
//         PropertyValue::Range(r.start().clone(), r.end().clone())
//     }
// }

#[derive(Debug, PartialEq)]
pub struct PropertyKeyframe<T> {
    value: Value<T>,
    // TODO: keyframe params
}

#[derive(Debug, PartialEq)]
pub enum ValueOp<T> {
    Add(Value<T>),
    Subtract(Value<T>),
    Multiply(Value<T>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Unit {
    Dimensionless, // (f32),

    Percent, //(u32),
    Px, //(u32),
    Pt,
    Em,
    Rem,
    In,
    Cm,
    Mm,
    Ex,
    Ch,
    Pc,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Deg,
    Rad,
    Turn,
}

impl Unit {
    fn from_value(text: &str) -> Option<Unit> {
        match UNIT_REGEX.captures(text)?.get(1)?.as_str() {
            "%" => Some(Unit::Percent),
            "px" => Some(Unit::Px),
            "pt" => Some(Unit::Pt),
            "em" => Some(Unit::Em),
            "rem" => Some(Unit::Rem),
            "in" => Some(Unit::In),
            "cm" => Some(Unit::Cm),
            "mm" => Some(Unit::Mm),
            "ex" => Some(Unit::Ex),
            "ch" => Some(Unit::Ch),
            "pc" => Some(Unit::Pc),
            "vw" => Some(Unit::Vw),
            "vh" => Some(Unit::Vh),
            "vmin" => Some(Unit::Vmin),
            "vmax" => Some(Unit::Vmax),
            "deg" => Some(Unit::Deg),
            "rad" => Some(Unit::Rad),
            "turn" => Some(Unit::Turn),
            _ => None,
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub enum Color {
//     Rgb,
//     Rgba,
//     Hsl,
//     Hsla,

//     Named,
//     Hex,
// }

pub type Color = palette::Alpha<palette::Srgb<u8>, f32>;

/* Get Relative Value, just replace with a closure?

function getRelativeValue(to, from) {
  var operator = /^(\*=|\+=|-=)/.exec(to);
  if (!operator) { return to; }
  var u = getUnit(to) || 0;
  var x = parseFloat(from);
  var y = parseFloat(to.replace(operator[0], ''));
  switch (operator[0][0]) {
    case '+': return x + y + u;
    case '-': return x - y + u;
    case '*': return x * y + u;
  }
}

*/

// type ColorValue = palette::Alpha<palette::Srgb<u8>, f32>;

// enum Value {
//     Color(ColorValue),
// }

// impl Value {
//     pub fn color<C: Into<ColorValue>>(c: C) -> Self {
//         Self::Color(c.into())
//     }
// }

// impl Display for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Value::Color(color) => {
//                 write!(
//                     f,
//                     "rgba({}, {}, {}, {})",
//                     color.red, color.green, color.blue, color.alpha
//                 )
//             }
//         }
//     }
// }

// TODO: remove
// pub enum OldValue<T> {
//     Dimensionless(T),

//     Unit(T, Unit), // ??

//     /// x += t
//     RelativeAdd(T),
//     /// x -= t
//     RelativeSub(T),
//     /// x *= t
//     RelativeMul(T),

//     ColorHex(String),
//     ColorRgb(u8, u8, u8),
//     ColorRgba(u8, u8, u8, f32),
//     ColorHsl(), // TODO palette?
//     ColorHsla(),

//     FromTo(Box<Value<T>>, Box<Value<T>>), // OR just force T?

//     Function(Box<dyn Fn(web_sys::Element, u32, u32) -> Value<T>>),
// }
