use rand::{distributions::uniform::SampleUniform, prelude::*};

pub const PI: f64 = std::f64::consts::PI;
pub type Degree = f64;
pub type Radian = f64;

/// 角度转弧度，180° = π
pub fn degree_to_radian(degree: Degree) -> Radian {
    degree * PI / 180.
}

/// 随机数，范围[0, 1)
pub fn random<>() -> f64 {
    // random number in range [0, 1)
    random_range(0., 1.)
}

/// 随机数，范围[min, max)
pub fn random_range<T>(min: T, max: T) -> T
    where T: SampleUniform + PartialOrd
{
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[cfg(test)]
mod tests {
    use crate::assert_f64_eq;
    use super::*;

    #[test]
    fn test_degree() {
        assert_f64_eq!(PI, degree_to_radian(180.));
    }

    #[test]
    fn test_random() {
        assert!(random() >= 0.);
        assert!(random() < 1.);
        assert!(random_range(1., 2.) >= 1.);
        assert!(random_range(1., 2.) < 2.);
    }
}
