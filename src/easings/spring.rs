const SPRING_PARAM_MIN: f32 = 0.1;
const SPRING_PARAM_MAX: f32 = 100.0;
const SPRING_SETTLING_THRESHOLD: f32 = 0.0001;
// const SPRING_SETTLING_THRESHOLD_LN: f32 = SPRING_SETTLING_THRESHOLD.ln(); // TODO: pre-calculate

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringProps {
    pub mass: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub initial_velocity: f32,
}

impl Default for SpringProps {
    fn default() -> Self {
        Self {
            mass: 1f32,
            stiffness: 100f32,
            damping: 10f32,
            initial_velocity: 0f32,
        }
    }
}

impl SpringProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mass(&mut self, m: f32) -> &mut Self {
        self.mass = m;
        self
    }
    pub fn stiffness(&mut self, s: f32) -> &mut Self {
        self.stiffness = s;
        self
    }
    pub fn damping(&mut self, d: f32) -> &mut Self {
        self.damping = d;
        self
    }
    pub fn initial_velocity(&mut self, v: f32) -> &mut Self {
        self.initial_velocity = v;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spring {
    /// Animation duration (in seconds)
    duration: f32,
    inner: SpringInner,
}

#[derive(Debug, Clone, PartialEq)]
enum SpringInner {
    Underdamped(UnderdampedSpring),
    CriticallyDamped(CriticallyDampedSpring),
    Overdamped(OverdampedSpring),
}

impl Spring {
    /// Create a new spring easing. Returns none if the damping ratio is not
    /// a real number.
    pub fn new(props: SpringProps) -> Option<Self> {
        let mass = props.mass.clamp(SPRING_PARAM_MIN, SPRING_PARAM_MAX);
        let stiffness = props.stiffness.clamp(SPRING_PARAM_MIN, SPRING_PARAM_MAX);
        let damping = props.damping.clamp(SPRING_PARAM_MIN, SPRING_PARAM_MAX);
        let initial_velocity = -props
            .initial_velocity
            .clamp(SPRING_PARAM_MIN, SPRING_PARAM_MAX);

        // Or ω0, ωn
        let natural_frequency = (stiffness / mass).sqrt();

        // Or ζ
        let damping_ratio = damping / (2.0 * (stiffness * mass).sqrt());

        use std::cmp::Ordering::*;
        let inner = match damping_ratio.partial_cmp(&1.0) {
            Some(Less) => {
                // Or ωd
                let damped_frequency = natural_frequency * (1.0 - damping_ratio.powi(2)).sqrt();
                SpringInner::Underdamped(UnderdampedSpring {
                    natural_frequency,
                    damping_ratio,
                    damped_frequency,

                    oscillation_const: (initial_velocity + (damping_ratio * natural_frequency))
                        / damped_frequency,
                })
            }
            Some(Equal) => SpringInner::CriticallyDamped(CriticallyDampedSpring {
                initial_velocity,
                natural_frequency,
            }),
            Some(Greater) => {
                // Or ωd
                let damped_frequency = natural_frequency * (damping_ratio.powi(2) - 1.0).sqrt();
                SpringInner::Overdamped(OverdampedSpring {
                    natural_frequency,
                    damping_ratio,
                    damped_frequency,

                    c1: (initial_velocity
                        + ((damping_ratio * natural_frequency) + damped_frequency))
                        / (2.0 * damped_frequency),

                    c2: (initial_velocity
                        + ((damping_ratio * natural_frequency) - damped_frequency))
                        / (2.0 * damped_frequency),
                })
            }
            None => return None,
        };

        Some(Spring {
            duration: inner.settling_time(),
            inner,
        })
    }

    /// Update the spring animation with a fractional progress
    pub(crate) fn update_animation(&self, progress: f32) -> f32 {
        self.inner.solve_x(progress * self.duration)
    }
}

impl SpringInner {
    /// Solve for position x as a function of time t in seconds, where for
    /// calculation, initial position x0 = 1, but the output range is reversed
    /// (so the "mass" travels from 0 to 1, rather than 1 to 0).
    ///
    /// Also, note that initial_velocity should be passed with the reverse range
    /// in mind, such that a positive velocity points from 0 to 1 and can be
    /// used normally in this calculation.
    ///
    /// See: https://www.brown.edu/Departments/Engineering/Courses/En4/Notes/vibrations_free_damped/vibrations_free_damped.htm
    fn solve_x(&self, t: f32) -> f32 {
        use SpringInner::*;
        match self {
            Underdamped(spring) => spring.solve_x(t),
            CriticallyDamped(spring) => spring.solve_x(t),
            Overdamped(spring) => spring.solve_x(t),
        }
    }

    /// Calculate duration of the animation
    ///
    /// Anime does this by running the solver loop until position with within
    /// the floating point noise value of 1 (i.e. solve_x(t) == 1f32) for 16/6
    /// frames (odd choice, but ok). We'll just use the settling time, since
    /// that seems eaiser.
    ///
    /// Regardless of dampedness, the bounding decay of the system is restricted
    /// by `(-damping_ratio * natural_freq * t).exp()` (often `a` in `solve_x`).
    /// Once this falls below [`SPRING_SETTLING_THRESHOLD`], we can consider the
    /// spring finished.
    // TODO: possible improvement, check movement pixels to settle when change
    // is less than 1 px?
    fn settling_time(&self) -> f32 {
        use SpringInner::*;
        match self {
            Underdamped(spring) => spring.settling_time(),
            CriticallyDamped(spring) => spring.settling_time(),
            Overdamped(spring) => spring.settling_time(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct UnderdampedSpring {
    natural_frequency: f32,
    damping_ratio: f32,
    damped_frequency: f32,

    oscillation_const: f32,
}

impl UnderdampedSpring {
    /// Solve for position x(t), where t is in seconds
    fn solve_x(&self, t: f32) -> f32 {
        let decay_part = (-self.damping_ratio * self.natural_frequency * t).exp();
        let (sin_part, cos_part) = (self.damped_frequency * t).sin_cos();

        decay_part * (cos_part + (self.oscillation_const * sin_part))
    }

    /// Solve for settling time within [`SPRING_SETTLING_THRESHOLD`]
    fn settling_time(&self) -> f32 {
        -SPRING_SETTLING_THRESHOLD.ln() / (self.damping_ratio * self.natural_frequency)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CriticallyDampedSpring {
    initial_velocity: f32,
    natural_frequency: f32,
}

impl CriticallyDampedSpring {
    /// Solve for position x(t), where t is in seconds
    fn solve_x(&self, t: f32) -> f32 {
        (1.0 + ((self.initial_velocity + self.natural_frequency) * t))
            * (-self.natural_frequency * t).exp()
    }

    /// Solve for settling time within [`SPRING_SETTLING_THRESHOLD`]
    fn settling_time(&self) -> f32 {
        -SPRING_SETTLING_THRESHOLD.ln() / self.natural_frequency
    }
}

#[derive(Debug, Clone, PartialEq)]
struct OverdampedSpring {
    natural_frequency: f32,
    damping_ratio: f32,
    damped_frequency: f32,

    c1: f32,
    c2: f32,
}

impl OverdampedSpring {
    /// Solve for position x(t), where t is in seconds
    fn solve_x(&self, t: f32) -> f32 {
        let decay_part = (-self.damping_ratio * self.natural_frequency * t).exp();
        let exp1 = (self.damped_frequency * t).exp();
        let exp2 = (-self.damped_frequency * t).exp();
        decay_part * ((self.c1 * exp1) - (self.c2 * exp2))
    }

    /// Solve for settling time within [`SPRING_SETTLING_THRESHOLD`]
    fn settling_time(&self) -> f32 {
        -SPRING_SETTLING_THRESHOLD.ln() / (self.damping_ratio * self.natural_frequency)
    }
}
