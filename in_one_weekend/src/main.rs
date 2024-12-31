use in_one_weekend::{color::*, ray::Ray};
use vector3::{Point3, Vec3};

fn main() {
    generate_image();
}

fn ray_color(ray: &Ray) -> Color {
    let t = 0.5 * (ray.direction().unit().y() + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn generate_image() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    // Calculate the image height, and ensure that it's at least 1.
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    const FOCAL_LENGTH: f64 = 1.;
    const VIEWPORT_HEIGHT: f64 = 2.;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from_x(VIEWPORT_WIDTH);
    let viewport_v = Vec3::from_y(-VIEWPORT_HEIGHT);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::from_z(FOCAL_LENGTH) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc  = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:<10}\r", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc
                + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);

            write_color(&pixel_color);
        }
    }
    eprint!("\r{:<30}\n", "Down.");
}
