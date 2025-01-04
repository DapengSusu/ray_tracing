use std::rc::Rc;

use vector3::{Point3, Vec3};
use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::Lambertian,
    sphere::Sphere,
    texture::ImageTexture
};

const IMAGE_FILENAME: &str = "the_next_week/asserts/earthmap.jpg";

pub fn earth() {
    let earth_texture = Rc::new(ImageTexture::new(IMAGE_FILENAME));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe = Rc::new(Sphere::new_stationary_sphere(
        Point3::zero(),
        2.,
        earth_surface
    ));

    Camera::new(
        16./9.,
        400,
        100,
        50,
        20.,
        Point3::from_z(12.),
        Point3::zero(),
        Vec3::from_y(1.),
        0.,
        10.
    ).render(&HittableList::from_hittable(globe));
}
