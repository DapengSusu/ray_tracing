pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };

/// 区间
#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Create the interval tightly enclosing the two input intervals.
    pub fn from_intervals(a: Self, b: Self) -> Self {
        Self { min: a.min.min(b.min), max: a.max.max(b.max) }
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
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }

    /// 通过给定值填充一个间隔
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta * 0.5;
        Self { min: self.min - padding, max: self.max + padding }
    }
}

impl Default for Interval {
    /// 区间默认为空
    fn default() -> Self {
        // Self { min: f64::INFINITY, max: f64::NEG_INFINITY }
        EMPTY
    }
}
