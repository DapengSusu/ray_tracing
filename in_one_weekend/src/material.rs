use crate::{color::Color, hittable::HitRecord, ray::Ray};
use vector3::extension::{random_unit_vector, reflect};

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
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(ray_in.direction(), &hit_record.normal);

        Some((Ray::new(hit_record.point, reflected), self.albedo))
    }
}
