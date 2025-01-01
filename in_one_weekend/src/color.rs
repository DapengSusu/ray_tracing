use utils::interval::Interval;

pub type Color = vector3::Vec3;

pub fn write_color(pixel_color: Color) {
    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    // Translate the [0,1] component values to the byte range [0,255].
    const INTERVAL: Interval = Interval { min: 0., max: 0.999 };
    let r_byte = (INTERVAL.clamp(r) * 256.) as u32;
    let g_byte = (INTERVAL.clamp(g) * 256.) as u32;
    let b_byte = (INTERVAL.clamp(b) * 256.) as u32;

    // Write out the pixel color components.
    print!("{} {} {}\n", r_byte, g_byte, b_byte);
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return f64::sqrt(linear_component);
    }
    0.
}
