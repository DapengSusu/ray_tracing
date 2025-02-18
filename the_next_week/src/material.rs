use std::rc::Rc;

use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::{SolidColor, Texture}};
use vector3::extension::{cos_theta, random_unit_vector, reflect, refract};
use utils::rtweekend::random;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Default)]
pub struct InvalidMaterial;

impl Material for InvalidMaterial {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Color)> {
        unimplemented!("InvalidMaterial scatter")
    }
}

pub struct Lambertian {
    texture: Rc<dyn Texture>
}

impl Lambertian {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self { texture: Rc::new(SolidColor::new(albedo)) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction, ray_in.time());
        let attenuation = self.texture.value(hit_record.u, hit_record.v, &hit_record.point);

        Some((scattered, attenuation))
    }
}

#[derive(Default)]
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
        let scattered = Ray::new(hit_record.point, reflected, ray_in.time());
        let attenuation = self.albedo;
        if scattered.direction().dot(&hit_record.normal) > 0. {
            return Some((scattered, attenuation));
        }
        None
    }
}

#[derive(Default)]
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
        let scattered = Ray::new(hit_record.point, direction, ray_in.time());
        let attenuation = Color::one();

        Some((scattered, attenuation))
    }
}
