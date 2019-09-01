
//use std::rc::Rc;
use std::sync::Arc;

use super::hit::{Hit, HitRecord};
use super::{Ray, Vec3};
use super::material::Material;

pub struct Sphere<'a> {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Material + 'a + Sync + Send>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            mat: mat,
        }
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            // try smaller t first
            let t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let point = r.point_at_param(t);
                let normal = &(r.point_at_param(t) - &self.center) / self.radius;

                return Some(HitRecord::new(t, point, normal, self.mat.clone()))
            }

            // try larger t
            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let point = r.point_at_param(t);
                let normal = (r.point_at_param(t) - &self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal, self.mat.clone()))
            }
        }

        return None;
    }
}
