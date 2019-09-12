
use std::f64::consts::PI;

use super::Vec3;
use super::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(v_fov:f64, aspect: f64) -> Self {
        let theta = v_fov * PI;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        Self {
            origin: Vec3::origin(),
            lower_left: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::with_values(
            self.origin.clone(),
            &self.lower_left + u*&self.horizontal + v*&self.vertical - &self.origin,
        )
    }
}
