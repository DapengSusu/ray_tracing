// todo: 这里min和max的默认值是不是反了？
pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };

pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

// todo: 这里min和max的默认值是不是反了？
impl Default for Interval {
    fn default() -> Self {
        Self { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}