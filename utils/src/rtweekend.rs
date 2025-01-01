use rand::prelude::*;

pub const PI: f64 = std::f64::consts::PI;
pub type Degree = f64;

#[allow(dead_code)] // todo: remove
trait FromDegree {
    fn to_radian(self) -> Self;
}

impl FromDegree for Degree {
    fn to_radian(self) -> Self {
        self * PI / 180.
    }
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    // random number in range [0, 1)
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[cfg(test)]
mod tests {
    use crate::assert_f64_eq;
    use super::*;

    #[test]
    fn test_degree() {
        assert_f64_eq!(PI, 180.0.to_radian());
    }

    #[test]
    fn test_random() {
        assert!(random() >= 0.);
        assert!(random() < 1.);
        assert!(random_range(1., 2.) >= 1.);
        assert!(random_range(1., 2.) < 2.);
    }
}
