use std::ops::{Add, Mul, Sub};

// TODO: reevaluate using https://github.com/linebender/kurbo in the future

// TODO: f32 vs f64?

// Implemention from the calculation in https://github.com/juliangarnier/anime
// which borrows from https://github.com/gre/bezier-easing

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezierSolver {
    curve: CubicBezier,

    // TODO: Test with BTreeMap?
    /// Precalculated Points and `t`s
    interpolants: Vec<(Point, f32)>,
}

/// An implementation of the Cubic Bezier curve, with control points
/// pre-decomposed to polynomial coefficients.
///
/// From the original equation, [here](https://en.wikipedia.org/wiki/B%C3%A9zier_curve)
/// Original use in https://github.com/gre/bezier-easing
///
/// `B(t) = (1-t)^3 P_0 + 3t(1-t)^2 P_1 + 3t^2 (1-t) P_2 + t^3 P_3`
///
/// we can extract coefficients `A`, `B`, and `C` when `P_0 = (0, 0)` and `P_3 = (1, 1)`
///
/// `B(t) = ((A * t + B) * t + C) * t`
///
/// and
///
/// `B'(t) = 3At^2 + 2Bt + C
///
/// where
///
/// `A = 1 - 3P_2 + 3P_1`
///
/// `B = 3P_2 - 6P_1`
///
/// `C = 3P_1`
///
/// The above equations for `B(t)` and `B'(t)` are also appropriate for
/// calculating only x and y, given those values (x1, x2, ... instead of points)
#[derive(Debug, Clone, PartialEq)]
pub(super) struct CubicBezier {
    a: Point,
    b: Point,
    c: Point,
    // p1: Point,
    // p2: Point,
}

const P_0: Point = Point { x: 0.0, y: 0.0 };
const P_3: Point = Point { x: 1.0, y: 1.0 };

const SAMPLE_TABLE_SIZE: usize = 11;
const SAMPLE_STEP_SIZE: f32 = 1.0 / ((SAMPLE_TABLE_SIZE - 1) as f32); // TODO: .recip() is not const?
const LAST_SAMPLE: usize = SAMPLE_TABLE_SIZE - 1;

/// Minimum slope at which to use the Newton-Raphson approach for calculating
/// a best guess
const NEWTON_RAPHSON_MIN_SLOPE: f32 = 0.001;
const NEWTON_RAPHSON_MAX_ITERS: usize = 4;
const BINARY_SUBDIVISION_MAX_ITERS: usize = 10;
const BINARY_SUBDIVISION_MAX_ERROR: f32 = 0.0000001;

impl CubicBezier {
    pub fn solver(x1: f32, y1: f32, x2: f32, y2: f32) -> CubicBezierSolver {
        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };
        let curve = Self {
            // p1: Point { x: x1, y: y1 },
            // p2: Point { x: x2, y: y2 },
            a: 1.0 - (3.0 * p2) + (3.0 * p1),
            b: (3.0 * p2) - (6.0 * p1),
            c: 3.0 * p1,
        };

        let interpolants = (0..SAMPLE_TABLE_SIZE)
            .map(|i| {
                if i == 0 {
                    (P_0, 0.0)
                } else if i == LAST_SAMPLE {
                    (P_3, 1.0)
                } else {
                    let t = i as f32 * SAMPLE_STEP_SIZE;
                    let p = curve.evaluate(t);
                    (p, t)
                }
            })
            .collect();

        CubicBezierSolver {
            curve,
            interpolants,
        }
    }

    /// Note: `t` here is not time, but the parametric for the bezier curve
    /// In the easing, `x` is fractional time, and `y` is fractional output.
    fn evaluate(&self, t: f32) -> Point {
        // TODO: or tangent lines?
        if t <= 0.0 {
            return P_0;
        }
        if t >= 1.0 {
            return P_3;
        }

        // let t_rem = 1.0 - t;
        // Exact, but P_0 and P_3 are constants
        // (t_rem.powi(3) * P_0) +
        //     (3.0 * t_rem.powi(2) * t * self.p1)
        //     + (3.0 * t_rem * t.powi(2) * self.p2)
        //     + (t.powi(3) * P_3)
        // (3.0 * t_rem.powi(2) * t * self.p1) + (3.0 * t_rem * t.powi(2) * self.p2) + (t.powi(3))
        ((((self.a * t) + self.b) * t) + self.c) * t
    }

    /// Same as `evaluate`, but only for the `x` coordinate
    fn evaluate_x(&self, t: f32) -> f32 {
        ((((self.a.x * t) + self.b.x) * t) + self.c.x) * t
    }
    /// Same as `evaluate`, but only for the `y` coordinate
    fn evaluate_y(&self, t: f32) -> f32 {
        ((((self.a.y * t) + self.b.y) * t) + self.c.y) * t
    }

    /// Same as `evaluate`, but for x'(t)
    fn evaluate_x_deriv(&self, t: f32) -> f32 {
        (3.0 * self.a.x * t * t) + (2.0 * self.b.x * t) + self.c.x
    }
}

impl CubicBezierSolver {
    pub(super) fn update_animation(&self, fraction: f32) -> f32 {
        let x = fraction;
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }

        // Find an interpolant range for t // TODO: cleaner loop?
        let a_iter = self.interpolants.iter();
        let mut b_iter = self.interpolants.iter();
        let _ = b_iter.next();

        let mut iter = a_iter.zip(b_iter);
        let (mut ta, mut tb, mut t_guess) = loop {
            if let Some(((pa, ta), (pb, tb))) = iter.next() {
                use std::cmp::Ordering::*;
                match (x.partial_cmp(&pa.x), x.partial_cmp(&pb.x)) {
                    // Saturate on ends (don't use greater than b, since that's the
                    // next loop!)
                    (Some(Less | Equal), _) => return pa.y,
                    (_, Some(Equal)) => return pb.y,

                    // Between the two (most times we end up here)
                    (Some(Greater), Some(Less)) => {
                        break (*ta, *tb, ((x - pa.x) / (pb.x - pa.x)) * (tb - ta));
                    }

                    // Otherwise continue
                    _other => {} // (None, _) | (_, None) | (None, None) => todo!(),
                }
            } else {
                // Reached the end, saturate
                return 1.0;
            }
        };

        // Try to improve our guess
        let mut dx_dt = self.curve.evaluate_x_deriv(t_guess);
        if dx_dt >= NEWTON_RAPHSON_MIN_SLOPE {
            let mut error;
            for i in 0..NEWTON_RAPHSON_MAX_ITERS {
                if i > 0 {
                    dx_dt = self.curve.evaluate_x_deriv(t_guess);
                    if dx_dt == 0.0 {
                        break;
                    }
                }
                error = self.curve.evaluate_x(t_guess) - x;
                t_guess -= error / dx_dt;
            }
        } else if dx_dt != 0.0 {
            let mut error;
            for i in 0..BINARY_SUBDIVISION_MAX_ITERS {
                if i > 0 {
                    t_guess = ta + ((tb - ta) / 2.0);
                }
                error = self.curve.evaluate_x(t_guess) - x;
                if error.abs() <= BINARY_SUBDIVISION_MAX_ERROR {
                    break;
                } else if error > 0.0 {
                    tb = t_guess;
                } else {
                    ta = t_guess;
                }
            }
        }
        // Otherwise (dx_dt == 0), just use the existing guess

        self.curve.evaluate_y(t_guess)
    }

    // fn lookup(&self, x: f32) -> LookupResult {
    //     let mut a_iter = self.interpolants.iter();
    //     let mut b_iter = self.interpolants.iter();
    //     let _ = b_iter.next();
    //     for ((pa, ta), (pb, tb)) in a_iter.zip(b_iter) {
    //         use std::cmp::Ordering::*;
    //         match (x.partial_cmp(&pa.x), x.partial_cmp(&pb.x)) {
    //             // Saturate on ends (don't use greater than b, since that's the
    //             // next loop!)
    //             (Some(Less | Equal), _) => return LookupResult::Exactly(*pa),
    //             (_, Some(Equal)) => return LookupResult::Exactly(*pb),

    //             // Between the two (most times we end up here)
    //             (Some(Greater), Some(Less)) => {
    //                 return LookupResult::Interpolated(((x - pa.x) / (pb.x - pa.x)) * (tb - ta))
    //             }

    //             // Otherwise continue
    //             _other => {} // (None, _) | (_, None) | (None, None) => todo!(),
    //         }
    //     }

    //     // Reached the end, saturate (should be fine to unwrap here, since
    //     // this isn't public. There are always items in `interpolants`)
    //     LookupResult::Exactly(self.interpolants.last().unwrap().0)
    // }
}

impl Mul<Point> for f32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<f32> for Point {
    type Output = Point;
    fn add(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<Point> for f32 {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self - rhs.x,
            y: self - rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// TODO: Bezier tests
