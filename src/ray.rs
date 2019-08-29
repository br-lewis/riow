
use super::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray{
            origin: Vec3::origin(),
            dir: Vec3::origin(),
        }
    }

    pub fn with_values(origin: Vec3, dir: Vec3) -> Self {
        Ray{
            origin: origin,
            dir: dir,
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn point_at_param(&self, t: f64) -> Vec3 {
        &self.origin + t * &self.dir
    }
}
