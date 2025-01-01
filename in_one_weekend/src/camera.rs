use crate::{
    color::{write_color, Color},
    hittable_list::HittableList,
    ray::Ray
};
use utils::{interval::Interval, rtweekend::random};
use vector3::{extension::random_unit_vector, Point3, Vec3};

#[derive(Default)]
pub struct Camera {
    // Ratio of image width over height
    pub aspect_ratio: f64,
    // Rendered image width in pixel count
    pub image_width: u32,
    // Count of random samples for each pixel
    pub samples_per_pixel : u32,
    // Maximum number of ray bounces into scene
    pub max_depth: u32,

    // Rendered image height
    image_height: u32,
    // Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    // Camera center
    center: Point3,
    // Location of pixel 0, 0
    pixel00_loc: Point3,
    // Offset to pixel to the right
    pixel_delta_u: Vec3,
    // Offset to pixel below
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(
            aspect_ratio: f64,
            image_width: u32,
            samples_per_pixel: u32,
            max_depth: u32
        ) -> Self {
        let mut camera = Self::default();

        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.samples_per_pixel = samples_per_pixel;
        camera.max_depth = max_depth;
        camera.image_height = (image_width as f64 / aspect_ratio) as u32;
        camera.image_height = if camera.image_height < 1 {
            1
        } else {
            camera.image_height
        };

        camera.pixel_samples_scale = 1. / (samples_per_pixel as f64);

        camera.center = Point3::zero();

        // Determine viewport dimensions.
        const FOCAL_LENGTH: f64 = 1.;
        const VIEWPORT_HEIGHT: f64 = 2.;
        let viewport_width = VIEWPORT_HEIGHT * aspect_ratio;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::from_x(viewport_width);
        let viewport_v = Vec3::from_y(-VIEWPORT_HEIGHT);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        camera.pixel_delta_u = viewport_u / camera.image_width as f64;
        camera.pixel_delta_v = viewport_v / camera.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera.center
            - Vec3::from_z(FOCAL_LENGTH) - viewport_u/2. - viewport_v/2.;
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
    fn ray(&self, i: u32, j: u32) ->Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        Ray::new(self.center, pixel_sample - self.center)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random() - 0.5, random() - 0.5, 0.)
    }

    fn ray_color(&self, ray: Ray, depth: u32, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }
        if let Some(hit_record) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
            let direction = hit_record.normal + random_unit_vector();
            return 0.5 * self.ray_color(Ray::new(hit_record.point, direction), depth-1, world)
        }
        let t = 0.5 * (ray.direction().unit().y() + 1.);
        (1. - t)*Color::one() + t*Color::new(0.5, 0.7, 1.)
    }
}
