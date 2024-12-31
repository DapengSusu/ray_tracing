pub const PI: f64 = std::f64::consts::PI;

pub type Degree = f64;

#[allow(dead_code)]
trait FromDegree {
    fn to_radian(self) -> Self;
}

impl FromDegree for Degree {
    fn to_radian(self) -> Self {
        self * PI / 180.
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_f64_eq;
    use super::*;

    #[test]
    fn test_degree() {
        assert_f64_eq!(PI, 180.0.to_radian());
    }
}
