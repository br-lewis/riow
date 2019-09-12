use std::f64::consts::PI;

use super::Ray;
use super::Vec3;

pub struct Camera {
    pos: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect: f64) -> Self {
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(&(&pos - &look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&v_up, &w));
        let v = Vec3::cross(&w, &u);

        let lower_left = &pos - half_width * &u - half_height * &v - &w;
        let horizontal = 2.0 * half_width * &u;
        let vertical = 2.0 * half_height * &v;
        Self {
            pos,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::with_values(
            self.pos.clone(),
            &self.lower_left + u * &self.horizontal + v * &self.vertical - &self.pos,
        )
    }
}
