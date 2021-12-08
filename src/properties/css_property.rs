use super::{Color, Property, PropertyValue, Value};
use crate::builder::Builder;

macro_rules! impl_css_properties {
    {
        $(
            $name:ident => $value_type:ty,
        )*
    } => {paste::paste!{

        /// A subset of CSS properties supported for animation
        ///
        /// See https://developer.mozilla.org/en-US/docs/Web/CSS/Reference
        #[derive(Debug, PartialEq)]
        pub enum CssProperty {
            $(
                $name(PropertyValue<$value_type>),
            )*
        }

        impl CssProperty {
            pub fn js_name(&self) -> &'static str {
                match self {
                    $(
                        CssProperty::$name(_) => stringify!([<$name:camel>]),
                    )*
                }
            }
        }

        impl Builder {
            $(
                pub fn [<$name:snake>]<T: Into<crate::builder::property::Property<$value_type>>>(&mut self, value: T) -> &mut Self {
                    let crate::builder::property::Property {
                        value,
                        params
                    } = value.into();
                    self.properties.push((
                        Property::CssProperty(CssProperty::$name(value)),
                        params
                    ));
                    self
                }
            )*
        }
    }};
}

enum CssValue {
    Angle(Angle)
}

/// Clockwise Angle
enum Angle {
    Degrees(f32),
    Gradians(f32),
    Radians(f32),
    Turns(f32),
}



impl_css_properties! {
    AccentColor => Color,
    BackgroundColor => Color,
    // BackgroundPositionX(PropertyValue<f32>),
    // BackgroundPositionY(PropertyValue<f32>),
    // BackgroundSize(PropertyValue<f32>),

    BlockSize => f32,

    BorderBottomLeftRadius => f32, // TODO: multiple values
    BorderBottomRightRadius => f32,

    BorderStartEndRadius => f32,
    BorderStartStartRadius => f32,
    BorderEndEndRadius => f32,
    BorderEndStartRadius => f32,

    BorderInlineColor => Color,
    BorderInlineWidth => f32,
    BorderInlineEndColor => Color,
    BorderInlineEndWidth => f32,
    BorderInlineStartColor => Color,
    BorderInlineStartWidth => f32,

    BorderBottomColor => Color,
    BorderBottomWidth => f32,
    BorderLeftColor => Color,
    BorderLeftWidth => f32,
    BorderRightColor => Color,
    BorderRightWidth => f32,
    BorderTopColor => Color,
    BorderTopWidth => f32,
    BorderTopLeftRadius => f32,
    BorderTopRightRadius => f32,

    BorderColor => Color,
    BorderRadius => f32,
    BorderSpacing => f32,
    BorderWidth => f32,

    Bottom => f32,

}

// Left(PropertyValue<i32>),
