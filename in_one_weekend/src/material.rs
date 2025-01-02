use crate::{color::Color, hittable::HitRecord, ray::Ray};
use utils::rtweekend::random;
use vector3::extension::{cos_theta, random_unit_vector, reflect, refract};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Default)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Color)> {
        Some((Ray::default(), Color::zero()))
    }
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((Ray::new(hit_record.point, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: fuzz.min(1.) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal)
            .normalize() + self.fuzz * random_unit_vector();
        let scattered = Ray::new(hit_record.point, reflected);
        if scattered.direction().dot(&hit_record.normal) > 0. {
            return Some((scattered, self.albedo));
        }
        None
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// 计算反射率
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1. - refraction_index) / (1. + refraction_index)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if hit_record.front_face {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().normalize();
        let cos_theta = cos_theta(&unit_direction, &hit_record.normal);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let direction = if ri * sin_theta > 1. ||
            Dielectric::reflectance(cos_theta, ri) > random() {
            reflect(&unit_direction, &hit_record.normal) // 反射
        } else {
            refract(&unit_direction, &hit_record.normal, ri) // 折射
        };

        Some((Ray::new(hit_record.point, direction), Color::one()))
    }
}
