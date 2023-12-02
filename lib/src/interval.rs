use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl From<Range<f64>> for Interval {
    fn from(value: Range<f64>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[allow(dead_code)]
impl Interval {
    pub const EMPTY: Self = Self {
        start: f64::INFINITY,
        end: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        start: f64::NEG_INFINITY,
        end: f64::INFINITY,
    };
    pub const CAMERA: Self = Self {
        start: 0.001,
        end: f64::INFINITY,
    };

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.start {
            self.start
        } else if x > self.end {
            self.end
        } else {
            x
        }
    }
}
