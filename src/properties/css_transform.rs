use super::{value::Unit, Property as Prop, PropertyParameters, PropertyValue, Value};
use crate::builder::{property::Property, Builder};

macro_rules! impl_css_transforms {
    {
        $(
            $name:ident => $value_type:ty {
                default_unit: $units:expr
            },
        )*
    } => {paste::paste!{

        #[derive(Debug, PartialEq)]
        pub enum CssTransform {
            $(
                $name(PropertyValue<$value_type>),
            )*
        }

        // TODO: fields with stringify!([<$name:snake>])
        impl CssTransform {
            // fn css_name(&self) -> &'static str {
            //     match self {
            //         $(
            //             CssTransform::$name(_) => $css_name,
            //         )*
            //     }
            // }

            pub fn js_name(&self) -> &'static str {
                match self {
                    $(
                        CssTransform::$name(_) => stringify!([<$name:camel>]),
                    )*
                }
            }

            pub fn default_units(&self) -> Option<Unit> {
                match self {
                    $(
                        CssTransform::$name(_) => $units,
                    )*
                }
            }
        }

        impl Builder {
            $(
                pub fn [<$name:snake>]<T: Into<Property<$value_type>>>(&mut self, value: T) -> &mut Self {
                    let crate::builder::property::Property {
                        value,
                        params
                    } = value.into();
                    self.properties.push((
                        Prop::CssTransform(CssTransform::$name(value)),
                        params
                    ));
                    self
                }
            )*
        }
    }};
}

use Unit::*;
impl_css_transforms! {
    TranslateX => i32 { default_unit: Some(Px) },
    TranslateY => i32 { default_unit: Some(Px) },
    TranslateZ => i32 { default_unit: Some(Px) },

    Rotate => i32 { default_unit: Some(Deg) },
    RotateX => i32 { default_unit:  Some(Deg) },
    RotateY => i32 { default_unit:  Some(Deg) },
    RotateZ => i32 { default_unit:  Some(Deg) },

    Scale => f32 { default_unit: Some(Dimensionless) },
    ScaleX => f32 { default_unit: Some(Dimensionless) },
    ScaleY => f32 { default_unit: Some(Dimensionless) },
    ScaleZ => f32 { default_unit: Some(Dimensionless) },

    Skew => i32 { default_unit: Some(Deg) },
    SkewX => i32 { default_unit: Some(Deg) },
    SkewY => i32 { default_unit: Some(Deg) },

    Perspective => i32 { default_unit: Some(Px) },

    Matrix => i32 { default_unit: None },
    Matrix3D => i32 { default_unit: None },
}
