use anime::prelude::*;
use palette::IntoColor;
use std::str::FromStr;

fn test() {
    let x = palette::Srgb::from_str("s");
}

// Make sure we can compile all of the "tutorial" cases
fn main() {
    // Animate CSS Properties
    animate("#target")
        .left((240, Unit::Px))
        .background_color(
            palette::Srgb::from_str("#FFF")
                .expect("Failed to interpret color")
                .into_color(),
        )
        .border_radius((0, Unit::Percent), (50, Unit::Percent))
        .easing(ease_in_out_quad())
        .finish(); // TODO: palette from_str

    // Animate CSS Transforms
    animate("#target")
        .translate_x(250)
        .scale(2.0)
        .rotate((1, Unit::Turn))
        .finish();

    // TODO: Object Props
    // TODO: Dom Attrs
    // TODO: Svg Attrs

    // Property Parameters
    // Duration
    animate("#target").translate_x(250).duration(3000).finish();

    // Delay
    animate("#target").translate_x(250).delay(1000).finish();

    // End Delay
    animate("#target")
        .translate_x(250)
        .end_delay(1000)
        .alternate()
        .finish();

    // Easing
    animate("#target")
        .translate_x(250)
        .easing(ease_in_out_expo())
        .finish();

    // Round // TODO

    // Specific Property Parameters // TODO: macro helper for property params?
    animate("#target")
        .translate_x(Property::value(250).duration(800))
        .rotate(
            Property::value(360)
                .duration(1800)
                .easing(ease_in_out_sine()),
        )
        .scale(
            Property::value(2.0)
                .duration(1600)
                .delay(800)
                .easing(ease_in_out_quart()),
        )
        .delay(250)
        .finish();

    // Function Based Parameters
    animate(".targets")
        .translate_x(270)
        .alternate()
        .repeat_forever()
        .delay_fn(|_elem, i, _n| (i * 100) as u32)
        .end_delay_fn(|_elem, i, n| ((n - i) * 100) as u32)
        .finish();

    // Animation Parameters
    // Direction
    // animate("#target")
    // .translate_x(270)

    // TODO: Animation Direction
    // TODO: Animation Loop
    // TODO: Animation Autoplay

    // Values
    // Unitless
    animate("#target")
        .translate_x(250) // Becomes 250px
        .rotate(540) // Becomes 540deg
        .finish();

    // Specific Unit
    animate("#target")
        .width((100, Unit::Percent))
        .easing(ease_in_out_quad())
        .alternate()
        .repeat_forever()
        .finish();

    // Animation Keyframes
    animate("#target")
        .keyframes(vec![
            AnimationKeyframe::new().props([
                Property::translate_x(5),
                Property::border_radius(10).delay(15),
            ]),
            AnimationKeyframe::new().props([Property::opacity(0.8)]),
        ])
        .finish();

    // Properyt Keyframes
    animate("#target")
        .translate_x(vec![
            PropertyKeyframe::new().value(5).delay(100),
            PropertyKeyframe::new().value(10),
        ])
        .finish();
}
