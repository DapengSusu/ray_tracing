use crate::Vec3;

pub fn cos_theta(uv: &Vec3, n: &Vec3) -> f64 {
    (-uv.dot(n)).min(1.)
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1., 1.);
        let sq = p.squared();
        if 1e-160 < sq && sq < 1. {
            return p / sq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let unit = random_unit_vector();

    // In the same hemisphere as the normal
    (unit.dot(normal) > 0.).then(|| unit).unwrap_or(-unit)
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * v.dot(n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let r_out_perp = etai_over_etat * (*uv + cos_theta(uv, n) * *n);
    let r_out_parallel = -((1. - r_out_perp.squared()).abs()).sqrt() * *n;

    r_out_perp + r_out_parallel
}
