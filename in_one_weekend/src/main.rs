use std::rc::Rc;

use in_one_weekend::{
    camera::Camera,
    color::Color,
    sphere::Sphere,
    hittable_list::HittableList,
    material::{Lambertian, Metal}
};
use vector3::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground)));
    world.add(Rc::new(Sphere::new(Point3::from_z(-1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right)));

    // Camera render
    Camera::new(
        16./9.,
        400,
        100,
        50
    ).render(&world);
}
