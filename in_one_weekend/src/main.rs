use in_one_weekend::color::*;

fn main() {
    generate_image();
}

fn generate_image() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:<10}\r", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.;
            let pixel_color = Color::new(r, g, b);

            write_color(&pixel_color);
        }
    }
    eprint!("\r{:<30}\n", "Down.");
}
