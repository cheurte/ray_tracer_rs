use crate::{
    rtweekend::random_int,
    vec3::{Point3, Vec3},
};
// #[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new(point_count: i32) -> Self {
        let mut ranvec: Vec<Vec3> = Vec::new();
        for _ in 0..point_count {
            ranvec.push(Vec3::random_interval(-1.0, 1.0).unit_vector());
        }
        Self {
            ranvec,
            perm_x: Self::perlin_generate_perm(point_count),
            perm_y: Self::perlin_generate_perm(point_count),
            perm_z: Self::perlin_generate_perm(point_count),
        }
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32; //
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        // let mut c = [[[0.0; 2]; 2]; 2];
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zeros(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 254) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(c, u, v, w)
    }
    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::from(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize].dot(weight_v);
                }
            }
        }
        accum
    }
    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (0..n - 1).rev() {
            let target = random_int(0, i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp
        }
    }
    fn perlin_generate_perm(point_count: i32) -> Vec<i32> {
        let mut p: Vec<i32> = Vec::new();
        for i in 0..point_count {
            p.push(i);
        }
        Self::permute(&mut p, point_count);
        p
    }
}
