mod bezier;
pub mod helpers;
mod spring;

pub use spring::SpringProps;

#[derive(Debug, Clone, PartialEq)]
pub enum Easing {
    Linear,

    Quad(Direction),
    Cubic(Direction),
    Quart(Direction),
    Quint(Direction),
    Expo(Direction),

    Sine(Direction),
    Circ(Direction),
    Back(Direction),
    Bounce(Direction),

    CubicBezier(bezier::CubicBezierSolver),

    Spring(spring::Spring),

    Elastic {
        amplitude: f32, // default 1
        period: f32,    // default 0.5
        direction: Direction,
    },

    Steps(u32),
    // Custom(Box<dyn Fn(f32) -> f32>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    In,
    Out,
    InOut,
    OutIn,
}

impl Direction {
    fn calculate<F: Fn(f32) -> f32>(&self, f: F, t: f32) -> f32 {
        match self {
            In => f(t),
            Out => 1.0 - f(1.0 - t),

            InOut if t < 0.5 => 0.5 * f(2.0 * t),
            InOut => 0.5 * f((-2.0 * t) + 2.0),

            OutIn if t < 0.5 => 0.5 * (1.0 - f(1.0 - (2.0 * t))),
            OutIn => 0.5 * (1.0 + f((2.0 * t) - 1.0)),
        }
    }
}

impl Easing {
    fn at(&self, fraction: f32) -> f32 {
        use Direction::*;
        use Easing::*;
        let t = fraction.clamp(0.0, 1.0);
        if t == 0.0 {
            return 0.0;
        }
        if (t - 1.0).abs() < f32::EPSILON {
            return 1.0;
        }
        match self {
            Linear => t,

            // TODO: check if we gain any performance writing these out manually?
            Quad(direction) => direction.calculate(|x| x.powi(2), t),
            Cubic(direction) => direction.calculate(|x| x.powi(3), t),
            Quart(direction) => direction.calculate(|x| x.powi(4), t),
            Quint(direction) => direction.calculate(|x| x.powi(5), t),
            Expo(direction) => direction.calculate(|x| ((10.0 * x) - 10.0).exp2(), t), // TODO: more efficient?

            // Flipped first quarter cosine curve
            Sine(In) => 1.0 - (t * std::f32::consts::FRAC_PI_2).cos(),

            // First quarter sine curve
            Sine(Out) => (t * std::f32::consts::FRAC_PI_2).sin(),

            // Negative half cosine (shifted for 0-1)
            Sine(InOut) => -((t * std::f32::consts::PI).cos() - 1.0) / 2.0,

            // Sine InOut flipped along y=x axis
            Sine(OutIn) => (1.0 - (2.0 * t)).acos() * std::f32::consts::FRAC_1_PI,

            // Quarter circle, concave up
            Circ(In) => 1.0 - (1.0 - (t * t)).sqrt(),

            // Quarter circle, concave down
            Circ(Out) => (1.0 - (t - 1.0).powi(2)).sqrt(),

            // Two Quarter circles (up-down)
            Circ(InOut) => {
                if t < 0.5 {
                    0.5 - (0.25 - (t * t)).sqrt()
                } else {
                    0.5 + (0.25 - (t - 1.0).powi(2)).sqrt()
                }
            }

            // Two Quarter circles (down-up)
            Circ(OutIn) => {
                if t < 0.5 {
                    (0.25 - (t - 0.5).powi(2)).sqrt()
                } else {
                    1.0 - (0.25 - (t - 0.5).powi(2)).sqrt()
                }
            }

            // Anime approximation of Penner's "Back" Easing
            Back(direction) => direction.calculate(|x| x * x * ((3.0 * x) - 2.0), t),

            // Penner's "Bounce" Easing (not exactly like above directions,
            // so we'll manually flip these)
            Bounce(In) => 1.0 - bounce_out(1.0 - t),
            Bounce(Out) => bounce_out(t),
            Bounce(InOut) => {
                if t < 0.5 {
                    0.5 - (0.5 * bounce_out(1.0 - (2.0 * t)))
                } else {
                    0.5 - (0.5 * bounce_out((2.0 - t) - 1.0))
                }
            }
            Bounce(OutIn) => {
                if t < 0.5 {
                    0.5 * bounce_out(2.0 * t)
                } else {
                    0.5 * bounce_out(2.0 - (2.0 * t))
                }
            }

            Elastic {
                amplitude,
                period,
                direction,
            } => direction.calculate(|x| elastic(*amplitude, *period, x), t),

            Steps(n) => {
                let n = *n as f32;
                (fraction * n).ceil() / n
            }
            Spring(spring) => spring.update_animation(fraction),

            CubicBezier(solver) => solver.update_animation(fraction),
            // Custom(f) => f(t),
        }
    }
}

const BACK_C1: f32 = 1.70158;
const BACK_C3: f32 = BACK_C1 + 1.0;

fn back_in(t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    (BACK_C3 * t3) - (BACK_C1 * t2)
}

fn back_out(t: f32) -> f32 {
    let t1 = t - 1.0;
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    1.0 + (BACK_C3 * t3) + (BACK_C1 * t2)
}

// TODO: BackInOut
// const c1 = 1.70158;
// const c2 = c1 * 1.525;

// return x < 0.5
//   ? (pow(2 * x, 2) * ((c2 + 1) * 2 * x - c2)) / 2
//   : (pow(2 * x - 2, 2) * ((c2 + 1) * (x * 2 - 2) + c2) + 2) / 2;

const BOUNCE_SCALE: f32 = 7.5625;

const BOUNCE_STEP: f32 = 2.75;

const BOUNCE_1_END: f32 = 1.0 / BOUNCE_STEP;
const BOUNCE_2_CENTER: f32 = 1.5 / BOUNCE_STEP;
const BOUNCE_2_END: f32 = 2.0 / BOUNCE_STEP;
const BOUNCE_3_CENTER: f32 = 2.25 / BOUNCE_STEP;
const BOUNCE_3_END: f32 = 2.5 / BOUNCE_STEP;
const BOUNCE_4_CENTER: f32 = 2.625 / BOUNCE_STEP;

const BOUNCE_2_OFFSET: f32 = 0.75;
const BOUNCE_3_OFFSET: f32 = 0.9375;
const BOUNCE_4_OFFSET: f32 = 0.984375;

fn bounce_out(mut t: f32) -> f32 {
    if t < BOUNCE_1_END {
        BOUNCE_SCALE * t * t
    } else if t < BOUNCE_2_END {
        t -= BOUNCE_2_CENTER;
        (BOUNCE_SCALE * t * t) + BOUNCE_2_OFFSET
    } else if t < BOUNCE_3_END {
        t -= BOUNCE_3_CENTER;
        (BOUNCE_SCALE * t * t) + BOUNCE_3_OFFSET
    } else {
        t -= BOUNCE_4_CENTER;
        (BOUNCE_SCALE * t * t) + BOUNCE_4_OFFSET
    }

    // TODO: Anime is slightly different. above is from https://easings.net/#easeOutBounce
    // Bounce: function () { return function (t) {
    //     var pow2, b = 4;
    //     while (t < (( pow2 = Math.pow(2, --b)) - 1) / 11) {}
    //     return 1 / Math.pow(4, 3 - b) - 7.5625 * Math.pow(( pow2 * 3 - 2 ) / 22 - t, 2)
    //   }; },
}

// const ELASTIC_PERIOD_SCALE: f32 = 2.0 * std::f32::consts::PI / 3.0;
// const ELASTIC_PERIOD_SHIFT: f32 = 10.75;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

fn elastic(amplitude: f32, period: f32, t: f32) -> f32 {
    // let decay_term = ((10.0 * t) - 10.0).exp2();
    // let oscillating_term = (((10.0 * t) - ELASTIC_PERIOD_SHIFT) * ELASTIC_PERIOD_SCALE).sin();
    // -decay_term * oscillating_term

    let t1 = t - 1.0;

    // TODO: return closure, based on precalculated params?
    let decay_term = (10.0 * t1).exp2();

    // y_peak = A*sin(x_peak) -> x_peak = arcsin(1/A) for y_peak = 1 (I think? Going off of anime maths)
    let period_shift = amplitude.recip().asin();
    // scale period so we get p periods in [0,1]
    let period_scale = TWO_PI / period;
    let oscillating_term = ((t1 * period_scale) - period_shift).sin();

    -amplitude * decay_term * oscillating_term
}
