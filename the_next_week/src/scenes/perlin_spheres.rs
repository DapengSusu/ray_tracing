use std::rc::Rc;

use vector3::{Point3, Vec3};
use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::Lambertian,
    sphere::Sphere,
    texture::NoiseTexture
};


pub fn perlin_spheres() {
    let mut world = HittableList::default();
    let perlin_texture = Rc::new(NoiseTexture::new());
    let perlin_material = Rc::new(Lambertian::new(perlin_texture));

    world.add(Rc::new(Sphere::new_stationary_sphere(
        Point3::from_y(-1000.),
        1000.,
        perlin_material.clone()
    )));

    world.add(Rc::new(Sphere::new_stationary_sphere(
        Point3::from_y(2.),
        2.,
        perlin_material
    )));

    Camera::new(
        16./9.,
        400,
        100,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::zero(),
        Vec3::from_y(1.),
        0.,
        10.
    ).render(&world);
}
