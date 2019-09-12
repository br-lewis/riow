use std::f64::consts::PI;

use rand::Rng;

use super::Ray;
use super::Vec3;

pub struct Camera {
    pos: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        pos: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(&(&pos - &look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        let v = Vec3::cross(&w, &u);

        let lower_left =
            &pos - half_width * focus_dist * &u - half_height * focus_dist * &v - focus_dist * &w;
        let horizontal = 2.0 * half_width * focus_dist * &u;
        let vertical = 2.0 * half_height * focus_dist * &v;
        Self {
            pos,
            lower_left,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();
        Ray::with_values(
            self.pos.clone() + &offset,
            &self.lower_left + s * &self.horizontal + t * &self.vertical - &self.pos - &offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if Vec3::dot(&v, &v) >- 1.0 {
            return v;
        }
    }
}