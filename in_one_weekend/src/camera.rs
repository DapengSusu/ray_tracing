use crate::{
    color::{write_color, Color},
    hittable_list::HittableList,
    ray::Ray
};
use utils::{interval::Interval, rtweekend::{degree_to_radian, random}};
use vector3::{Point3, Vec3};

#[derive(Default)]
pub struct Camera {
    // Ratio of image width over height
    pub aspect_ratio: f64,
    // Rendered image width in pixel count
    pub image_width: i32,
    // Count of random samples for each pixel
    pub samples_per_pixel : i32,
    // Maximum number of ray bounces into scene
    pub max_depth: i32,

    // Vertical view angle (field of view)
    pub vertical_fov: f64,
    // Point camera is looking from
    pub look_from: Point3,
    // Point camera is looking at
    pub look_at: Point3,
    // Camera-relative "up" direction
    pub vup: Vec3,

    // Rendered image height
    image_height: i32,
    // Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    // Camera center
    center: Point3,
    // Location of pixel 0, 0
    pixel00_loc: Point3,
    // Offset to pixel to the right
    pixel_delta_u: Vec3,
    // Offset to pixel below
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3
}

impl Camera {
    pub fn new(
            aspect_ratio: f64,
            image_width: i32,
            samples_per_pixel: i32,
            max_depth: i32,
            vertical_fov: f64,
            look_from: Point3,
            look_at: Point3,
            vup: Vec3
        ) -> Self {
        let mut camera = Self::default();

        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.samples_per_pixel = samples_per_pixel;
        camera.max_depth = max_depth;
        camera.vertical_fov = vertical_fov;
        camera.look_from = look_from;
        camera.look_at = look_at;
        camera.vup = vup;
        camera.image_height = (image_width as f64 / aspect_ratio) as i32;
        camera.image_height = camera.image_height.max(1);

        camera.pixel_samples_scale = (samples_per_pixel as f64).recip();

        camera.center = look_from;

        // Determine viewport dimensions.
        let focal_length: f64 = (look_from - look_at).norm();
        let theta = degree_to_radian(camera.vertical_fov);
        let viewport_height: f64 = (theta / 2.).tan() * 2. * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        camera.w = (look_from - look_at).normalize();
        camera.u = vup.cross(&camera.w).normalize();
        camera.v = camera.w.cross(&camera.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        // Vector across viewport horizontal edge
        let viewport_u = viewport_width * camera.u;
        // Vector down viewport vertical edge
        let viewport_v = viewport_height * (-camera.v);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        camera.pixel_delta_u = viewport_u / camera.image_width as f64;
        camera.pixel_delta_v = viewport_v / camera.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera.center - focal_length * camera.w - (viewport_u + viewport_v) * 0.5;
        camera.pixel00_loc  = viewport_upper_left + 0.5 * (camera.pixel_delta_u + camera.pixel_delta_v);

        camera
    }

    pub fn render(&self, world: &HittableList) {
        // Render
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:<10}\r", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    pixel_color += self.ray_color(self.ray(i, j), self.max_depth, world);
                }

                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        eprint!("\r{:<30}\n", "Down.");
    }

    // Construct a camera ray originating from the origin and directed at randomly sampled
    // point around the pixel location i, j.
    fn ray(&self, i: i32, j: i32) ->Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random() - 0.5, random() - 0.5, 0.)
    }

    fn ray_color(&self, ray: Ray, depth: i32, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }
        if let Some(hit_record) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = hit_record.material.scatter(&ray, &hit_record) {
                return attenuation * self.ray_color(scattered, depth-1, world);
            }
            return Color::zero();
        }
        let t = 0.5 * (ray.direction().normalize().y + 1.);
        (1. - t)*Color::one() + t*Color::new(0.5, 0.7, 1.)
    }
}
