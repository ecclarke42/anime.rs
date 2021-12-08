mod css_property;
mod css_transform;
mod dom_attribute;
mod svg_attribute;
mod value;

pub use css_property::CssProperty;
pub use css_transform::CssTransform;
pub use dom_attribute::DomAttribute;
pub use svg_attribute::SvgAttribute;
pub use value::{Color, PropertyValue, Unit, Value, ValueOp};

use crate::easings::Easing; // TODO: un-module these?

/// An animatable property value
#[derive(Debug, PartialEq)]
pub enum Property {
    CssProperty(CssProperty),
    CssTransform(CssTransform),
    // TODO
    // ObjectField,
    // DomAttribute(DomAttribute),
    // SvgAttribute(SvgAttribute),
}

impl From<CssProperty> for Property {
    fn from(p: CssProperty) -> Self {
        Self::CssProperty(p)
    }
}
impl From<CssTransform> for Property {
    fn from(p: CssTransform) -> Self {
        Self::CssTransform(p)
    }
}
// impl From<DomAttribute> for Property {
//     fn from(p: DomAttribute) -> Self {
//         Self::DomAttribute(p)
//     }
// }
// impl From<SvgAttribute> for Property {
//     fn from(p: SvgAttribute) -> Self {
//         Self::SvgAttribute(p)
//     }
// }

// pub trait Animatable {
//     fn update_value(&mut self, value: Value);
//     fn apply_to_target(&self, target: &Target);
// }

// TODO: helper?
// macro_rules! animation {
//     {
//         $($prop:ident: $value:expr)
//     } => {

//     };
// }

// TODO: parameters mod?

pub struct PropertyParameters<T> {
    value: PropertyValue<T>,

    duration: u32, // default 1000
    delay: u32,
    end_delay: u32,
    easing: Easing,
    round: u32,
}
