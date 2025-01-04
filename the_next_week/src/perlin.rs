use utils::rtweekend::{random, random_range};
use vector3::Point3;

const POINT_COUNT: usize = 256;
type Perm = [usize; POINT_COUNT];

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: Perm,
    perm_y: Perm,
    perm_z: Perm
}

impl Perlin {

    pub fn new() -> Self {
        let mut perlin = Self::default();

        for i in 0..POINT_COUNT {
            perlin.randfloat[i] = random();
        }
        Self::perlin_generate_perm(&mut perlin.perm_x);
        Self::perlin_generate_perm(&mut perlin.perm_y);
        Self::perlin_generate_perm(&mut perlin.perm_z);

        perlin
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.*p.x) as usize & 255;
        let j = (4.*p.y) as usize & 255;
        let k = (4.*p.z) as usize & 255;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(perm: &mut Perm) {
        for i in 0..POINT_COUNT {
            perm[i] = i;
        }
        Self::permute(perm, POINT_COUNT);
    }

    fn permute(perm: &mut Perm, n: usize) {
        for i in (1..n).rev() {
            let target = random_range::<usize>(0, i);

            perm.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            randfloat: [0.; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT]
        }
    }
}
