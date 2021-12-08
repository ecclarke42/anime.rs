use super::{
    bezier::CubicBezier,
    spring::{Spring, SpringProps},
    Direction, Easing,
};

/// Linear Easing
pub fn linear() -> Easing {
    Easing::Linear
}

/// Stepped easing
pub fn steps(n: u32) -> Easing {
    Easing::Steps(n)
}

/// Parameterized Cubic Bezier Easing
pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Easing {
    Easing::CubicBezier(CubicBezier::solver(x1, y1, x2, y2))
}

/// Cubic Bezier curve represented by css keyword `ease`
///
/// Same as `Easing::bezier(0.25, 0.1, 0.25, 1.0)`
pub fn ease() -> Easing {
    Easing::CubicBezier(CubicBezier::solver(0.25, 0.1, 0.25, 1.0))
}
/// Cubic Bezier curve represented by css keyword `ease-in`
///
/// Same as `Easing::bezier(0.42, 0.0, 1.0, 1.0)`
pub fn ease_in() -> Easing {
    Easing::CubicBezier(CubicBezier::solver(0.42, 0.0, 1.0, 1.0))
}
/// Cubic Bezier curve represented by css keyword `ease-in-out`
///
/// Same as `Easing::bezier(0.42, 0.0, 0.58, 1.0)`
pub fn ease_in_out() -> Easing {
    Easing::CubicBezier(CubicBezier::solver(0.42, 0.0, 0.58, 1.0))
}
/// Cubic Bezier curve represented by css keyword `ease-out`
///
/// Same as `Easing::bezier(0.0, 0.0, 0.58, 1.0)`
pub fn ease_out() -> Easing {
    Easing::CubicBezier(CubicBezier::solver(0.0, 0.0, 0.58, 1.0))
}

pub fn elastic_in(amplitude: f32, period: f32) -> Easing {
    elastic(amplitude, period, Direction::In)
}
pub fn elastic_out(amplitude: f32, period: f32) -> Easing {
    elastic(amplitude, period, Direction::Out)
}
pub fn elastic_in_out(amplitude: f32, period: f32) -> Easing {
    elastic(amplitude, period, Direction::InOut)
}
pub fn elastic_out_in(amplitude: f32, period: f32) -> Easing {
    elastic(amplitude, period, Direction::OutIn)
}
fn elastic(amplitude: f32, period: f32, direction: Direction) -> Easing {
    Easing::Elastic {
        amplitude: amplitude.clamp(1.0, 10.0),
        period: period.clamp(0.1, 2.0),
        direction,
    }
}

pub fn spring(mass: f32, stiffness: f32, damping: f32, initial_velocity: f32) -> Option<Easing> {
    Spring::new(SpringProps {
        mass,
        stiffness,
        damping,
        initial_velocity,
    })
    .map(|s| Easing::Spring(s))
}

/// Quad Easing In
pub fn ease_in_quad() -> Easing {
    Easing::Quad(Direction::In)
}
/// Quad Easing Out
pub fn ease_out_quad() -> Easing {
    Easing::Quad(Direction::Out)
}
/// Quad Easing In Out
pub fn ease_in_out_quad() -> Easing {
    Easing::Quad(Direction::InOut)
}
/// Quad Easing Out In
pub fn ease_out_in_quad() -> Easing {
    Easing::Quad(Direction::OutIn)
}
/// Cubic Easing In
pub fn ease_in_cubic() -> Easing {
    Easing::Cubic(Direction::In)
}
/// Cubic Easing Out
pub fn ease_out_cubic() -> Easing {
    Easing::Cubic(Direction::Out)
}
/// Cubic Easing In Out
pub fn ease_in_out_cubic() -> Easing {
    Easing::Cubic(Direction::InOut)
}
/// Cubic Easing Out In
pub fn ease_out_in_cubic() -> Easing {
    Easing::Cubic(Direction::OutIn)
}
/// Quart Easing In
pub fn ease_in_quart() -> Easing {
    Easing::Quart(Direction::In)
}
/// Quart Easing Out
pub fn ease_out_quart() -> Easing {
    Easing::Quart(Direction::Out)
}
/// Quart Easing In Out
pub fn ease_in_out_quart() -> Easing {
    Easing::Quart(Direction::InOut)
}
/// Quart Easing Out In
pub fn ease_out_in_quart() -> Easing {
    Easing::Quart(Direction::OutIn)
}
/// Quint Easing In
pub fn ease_in_quint() -> Easing {
    Easing::Quint(Direction::In)
}
/// Quint Easing Out
pub fn ease_out_quint() -> Easing {
    Easing::Quint(Direction::Out)
}
/// Quint Easing In Out
pub fn ease_in_out_quint() -> Easing {
    Easing::Quint(Direction::InOut)
}
/// Quint Easing Out In
pub fn ease_out_in_quint() -> Easing {
    Easing::Quint(Direction::OutIn)
}
/// Expo Easing In
pub fn ease_in_expo() -> Easing {
    Easing::Expo(Direction::In)
}
/// Expo Easing Out
pub fn ease_out_expo() -> Easing {
    Easing::Expo(Direction::Out)
}
/// Expo Easing In Out
pub fn ease_in_out_expo() -> Easing {
    Easing::Expo(Direction::InOut)
}
/// Expo Easing Out In
pub fn ease_out_in_expo() -> Easing {
    Easing::Expo(Direction::OutIn)
}
/// Sine Easing In
pub fn ease_in_sine() -> Easing {
    Easing::Sine(Direction::In)
}
/// Sine Easing Out
pub fn ease_out_sine() -> Easing {
    Easing::Sine(Direction::Out)
}
/// Sine Easing In Out
pub fn ease_in_out_sine() -> Easing {
    Easing::Sine(Direction::InOut)
}
/// Sine Easing Out In
pub fn ease_out_in_sine() -> Easing {
    Easing::Sine(Direction::OutIn)
}
/// Circ Easing In
pub fn ease_in_circ() -> Easing {
    Easing::Circ(Direction::In)
}
/// Circ Easing Out
pub fn ease_out_circ() -> Easing {
    Easing::Circ(Direction::Out)
}
/// Circ Easing In Out
pub fn ease_in_out_circ() -> Easing {
    Easing::Circ(Direction::InOut)
}
/// Circ Easing Out In
pub fn ease_out_in_circ() -> Easing {
    Easing::Circ(Direction::OutIn)
}
/// Back Easing In
pub fn ease_in_back() -> Easing {
    Easing::Back(Direction::In)
}
/// Back Easing Out
pub fn ease_out_back() -> Easing {
    Easing::Back(Direction::Out)
}
/// Back Easing In Out
pub fn ease_in_out_back() -> Easing {
    Easing::Back(Direction::InOut)
}
/// Back Easing Out In
pub fn ease_out_in_back() -> Easing {
    Easing::Back(Direction::OutIn)
}
/// Bounce Easing In
pub fn ease_in_bounce() -> Easing {
    Easing::Bounce(Direction::In)
}
/// Bounce Easing Out
pub fn ease_out_bounce() -> Easing {
    Easing::Bounce(Direction::Out)
}
/// Bounce Easing In Out
pub fn ease_in_out_bounce() -> Easing {
    Easing::Bounce(Direction::InOut)
}
/// Bounce Easing Out In
pub fn ease_out_in_bounce() -> Easing {
    Easing::Bounce(Direction::OutIn)
}

// pub fn custom<F: Fn(f32) -> f32>(f: F) -> Easing {
//     Easing::Custom(Box::new(f))
// }
