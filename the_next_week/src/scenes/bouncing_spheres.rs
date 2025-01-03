use std::rc::Rc;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    texture::CheckerTexture
};
use utils::rtweekend::{random, random_range};
use vector3::{Point3, Vec3};

pub fn bouncing_spheres() {
    // World
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::isotropic(0.9)
    ));
    world.add(Rc::new(Sphere::new_stationary_sphere(
        Point3::from_y(-1000.),
        1000.,
        Rc::new(Lambertian::new(checker.clone()))
    )));

    // let ground_material = Rc::new(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)));
    // world.add(Rc::new(Sphere::new_stationary_sphere(Point3::from_y(-1000.), 1000., ground_material)));

    generate_sphere_random(&mut world);

    let material_1 = Rc::new(Dielectric::new(1.5));
    let material_2 = Rc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));

    world.add(Rc::new(Sphere::new_stationary_sphere(Point3::from_y(1.), 1., material_1)));
    world.add(Rc::new(Sphere::new_stationary_sphere(Point3::new(-4., 1., 0.), 1., material_2)));
    world.add(Rc::new(Sphere::new_stationary_sphere(Point3::new(4., 1., 0.), 1., material_3)));

    world = HittableList::from_hittable(Rc::new(BVHNode::from_hittable_list(&mut world)));

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
        0.6,
        10.
    ).render(&world);
}

fn generate_sphere_random(world: &mut HittableList) {
    for j in -10..11 {
        for i in -10..11 {
            let which_material = random();
            let center = Point3::new(
                j as f64 + 0.9 * random(),
                0.2,
                i as f64 + 0.9 * random()
            );

            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                if which_material < 0.7 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::from_color(albedo));
                    let end = center + Vec3::from_y(random_range(0., 0.5));
                    world.add(Rc::new(Sphere::new_moving_sphere(center, end, 0.2, material)));
                } else if which_material < 0.9 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_range(0., 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new_stationary_sphere(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new_stationary_sphere(center, 0.2, material)));
                }
            }
        }
    }
}
