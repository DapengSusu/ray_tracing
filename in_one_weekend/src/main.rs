use in_one_weekend::{
    hittable_list::HittableList,
    camera::Camera,
    sphere::Sphere
};
use vector3::Point3;
use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::from_z(-1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera render
    Camera::new(
        16./9.,
        400,
        100
    ).render(&world);
}
