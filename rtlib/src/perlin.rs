use super::vec3::{self, Vec3};
use super::vect;
use math::round;
use rand::Rng;

const MAX_PERLIN_PERM: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: Box<[Vec3]>,
    perm_x: Box<[i64]>,
    perm_y: Box<[i64]>,
    perm_z: Box<[i64]>,
}

// Perlin should be read only after construction
unsafe impl Send for Perlin {}
unsafe impl Sync for Perlin {}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: Perlin::perlin_generate(),
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let i: i64 = round::floor(p.x, 0) as i64;
        let j: i64 = round::floor(p.y, 0) as i64;
        let k: i64 = round::floor(p.z, 0) as i64;

        let u: f64 = p.x - i as f64;
        let v: f64 = p.y - j as f64;
        let w: f64 = p.z - k as f64;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..c.len() {
            for dj in 0..c[0].len() {
                for dk in 0..c[0][0].len() {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize])
                        as usize];
                }
            }
        }

        return trilinear_interp(&c, u, v, w);
    }

    pub fn perlin_generate() -> Box<[Vec3]> {
        let mut p: Vec<Vec3> = Vec::with_capacity(MAX_PERLIN_PERM);
        let mut rng = rand::thread_rng();

        for _i in 0..MAX_PERLIN_PERM {
            p.push(vec3::unit_vector(&vect!(
                -1. + 2. * rng.gen::<f64>(),
                -1. + 2. * rng.gen::<f64>(),
                -1. + 2. * rng.gen::<f64>()
            )));
        }

        p.into_boxed_slice()
    }

    pub fn permute(p: &mut Vec<i64>) {
        let mut rng = rand::thread_rng();
        for i in (0..(p.len() - 1)).rev() {
            let target: usize = (rng.gen::<f64>() * ((i + 1) as f64)) as usize;
            p.swap(i, target);
        }
    }

    pub fn perlin_generate_perm() -> Box<[i64]> {
        let mut p: Vec<i64> = Vec::with_capacity(MAX_PERLIN_PERM);

        for i in 0..MAX_PERLIN_PERM {
            p.push(i as i64);
        }
        Self::permute(&mut p);
        p.into_boxed_slice()
    }

    pub fn turbulance(&self, p: &Vec3, depth: u8) -> f64 {
        let mut accum: f64 = 0.;
        let mut temp_p: Vec3 = p.clone();
        let mut weight: f64 = 1.;
        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Perlin {
        Perlin::new()
    }
}

// this may have in fact become perlin_interp rather than the trilinear it used to be.
pub fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum: f64 = 0.;
    let uu: f64 = u * u * (3. - 2. * u);
    let vv: f64 = v * v * (3. - 2. * v);
    let ww: f64 = w * w * (3. - 2. * w);
    for i in 0..c.len() {
        for j in 0..c[0].len() {
            for k in 0..c[0][0].len() {
                let weight_v: Vec3 = vect!(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * vec3::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::Perlin;
    #[allow(unused_imports)]
    use rand::Rng;

    #[test]
    fn test_permute() {
        let mut bxslice: Vec<i64> = Vec::with_capacity(super::MAX_PERLIN_PERM);

        for i in 0..super::MAX_PERLIN_PERM {
            bxslice.push(i as i64);
        }

        let mut perm_bxslice = bxslice.clone();

        Perlin::permute(&mut perm_bxslice);

        assert_ne!(bxslice, perm_bxslice);
    }
}
