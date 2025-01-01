use crate::Vec3;

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1., 1.);
        let sq = p.squared();
        if 1e-160 < sq && sq < 1. {
            return p / f64::sqrt(sq);
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let unit = random_unit_vector();
    // In the same hemisphere as the normal
    if unit.dot(normal) > 0. {
        return unit;
    }
    -unit
}
