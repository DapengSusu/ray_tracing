use std::rc::Rc;

use in_one_weekend::{
    camera::Camera,
    color::Color,
    sphere::Sphere,
    hittable_list::HittableList,
    material::{Lambertian, Metal, Dielectric}
};
use utils::rtweekend::{random, random_range};
use vector3::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::from_y(-1000.), 1000., ground_material)));

    generate_sphere_random(&mut world);

    let material_1 = Rc::new(Dielectric::new(1.5));
    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Rc::new(Sphere::new(Point3::from_y(1.), 1., material_1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material_2)));
    world.add(Rc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material_3)));

    // Camera render
    Camera::new(
        16./9.,
        1200,
        500,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::zero(),
        Point3::from_y(1.),
        0.6,
        10.
    ).render(&world);
}

fn generate_sphere_random(world: &mut HittableList) {
    for j in -11..11 {
        for i in -11..11 {
            let which_material = random();
            let center = Point3::new(
                j as f64 + 0.9 * random(),
                0.2,
                i as f64 + 0.9 * random()
            );

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                if which_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else if which_material < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }
}
