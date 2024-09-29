use crate::{
    interval::Interval,
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Perlin {
    point_count: usize,
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Self {
        let rand_float = (0..point_count)
            .map(|_| Vec3::random_in_interval(Interval::from((-1.0, 1.0))))
            .collect();
        let perm_x = Self::generate_perm(point_count);
        let perm_y = Self::generate_perm(point_count);
        let perm_z = Self::generate_perm(point_count);
        Self {
            point_count,
            rand_vec: rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Point3) -> f64 {
        let limit = (self.point_count - 1) as isize;

        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as isize;
        let j = point.y().floor() as isize;
        let k = point.z().floor() as isize;

        let mut m = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    m[di][dj][dk] = self.rand_vec[self.perm_x[((i + di as isize) & limit) as usize]
                        ^ self.perm_y[((j + dj as isize) & limit) as usize]
                        ^ self.perm_z[((k + dk as isize) & limit) as usize]]
                }
            }
        }

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * m[i][j][k].dot(weight_v);
                }
            }
        }

        acc
    }

    pub fn turbulence(&self, mut point: Point3, iterations: usize) -> f64 {
        let mut acc = 0.0;
        let mut weight = 1.0;
        for _ in 0..iterations {
            acc += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.0;
        }
        acc.abs()
    }

    fn generate_perm(point_count: usize) -> Vec<usize> {
        let mut vec: Vec<usize> = (0..point_count).collect();
        fastrand::shuffle(&mut vec);
        vec
    }
}
