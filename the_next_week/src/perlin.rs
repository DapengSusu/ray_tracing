use utils::rtweekend::{random, random_range};
use vector3::Point3;

const POINT_COUNT: usize = 256;
type PermArr = [usize; POINT_COUNT];
type TriArr = [[[f64; 2]; 2]; 2];

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: PermArr,
    perm_y: PermArr,
    perm_z: PermArr
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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let mut c: TriArr = [[[0.; 2]; 2]; 2];
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                        self.perm_x[(i+di) & 255] ^
                        self.perm_y[(j+dj) & 255] ^
                        self.perm_z[(k+dk) & 255]
                    ];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn perlin_generate_perm(perm: &mut PermArr) {
        for i in 0..POINT_COUNT {
            perm[i] = i;
        }
        Self::permute(perm, POINT_COUNT);
    }

    fn permute(perm: &mut PermArr, n: usize) {
        for i in (1..n).rev() {
            let target = random_range::<usize>(0, i);

            perm.swap(i, target);
        }
    }

    fn trilinear_interp(c: &TriArr, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1-i) as f64 * (1.-u))
                           * (j as f64 * v + (1-j) as f64 * (1.-v))
                           * (k as f64 * w + (1-k) as f64 * (1.-w))
                           * c[i][j][k];
                }
            }
        }

        accum
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
