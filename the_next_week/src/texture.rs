use std::rc::Rc;

use crate::color::Color;
use vector3::Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Color::new(r, g, b) }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { inv_scale: scale.recip(), even, odd }
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(scale, Rc::new(SolidColor::new(c1)), Rc::new(SolidColor::new(c2)))
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;
        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even { self.even.value(u, v, p) } else { self.odd.value(u, v, p) }
    }
}
