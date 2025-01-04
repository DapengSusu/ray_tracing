/// 使用RGB格式，每个像素3个字节
const BYTES_PER_PIXEL: u32 = 3;

#[derive(Default)]
pub struct RTWImage {
    rgb_image: image::RgbImage,
    /// 每行像素的字节数
    bytes_per_scanline: u32,
    /// 图片宽度
    image_width: u32,
    /// 图片高度
    image_height: u32,
    /// 图片像素数据
    bdata: Vec<[u8; 3]>
}

impl RTWImage {
    pub fn new(image_filename: &str) -> RTWImage {
        Self::default().load(image_filename)
    }

    pub fn load(mut self, image_filename: &str) -> RTWImage {
        self.rgb_image = image::open(image_filename).unwrap().to_rgb8();

        (self.image_width, self.image_height) = self.rgb_image.dimensions();
        self.bytes_per_scanline = self.image_width * BYTES_PER_PIXEL;
        self.rgb_image.pixels().for_each(|pixel| {
            let rgb = image::Rgb([pixel.0[0], pixel.0[1], pixel.0[2]]);
            self.bdata.push(rgb.0);
        });

        self
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        let rgb: [u8; 3] = [255, 0, 255];
        if self.bdata.is_empty() {
            return rgb;
        }

        let pixel = self.rgb_image.get_pixel(x, y);

        pixel.0
    }

    pub fn width(&self) -> u32 {
        self.image_width
    }

    pub fn height(&self) -> u32 {
        self.image_height
    }
}
