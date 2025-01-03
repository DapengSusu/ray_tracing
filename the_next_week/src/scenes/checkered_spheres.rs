use std::rc::Rc;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::Lambertian,
    sphere::Sphere,
    texture::CheckerTexture
};
use vector3::Point3;

pub fn checkered_spheres() {
    // World
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::isotropic(0.9)
    ));
    world.add(Rc::new(Sphere::new_stationary_sphere(
        Point3::from_y(-10.),
        10.,
        Rc::new(Lambertian::new(checker.clone()))
    )));
    world.add(Rc::new(Sphere::new_stationary_sphere(
        Point3::from_y(10.),
        10.,
        Rc::new(Lambertian::new(checker.clone()))
    )));

    // Camera render
    Camera::new(
        16./9.,
        400,
        100,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::zero(),
        Point3::from_y(1.),
        0.,
        10.
    ).render(&world);
}
