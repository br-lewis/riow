use super::hit::{Hit, HitRecord};
use super::{Ray, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Vec3::origin(),
            radius: 0.0,
        }
    }

    pub fn with_vals(center: Vec3, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hit for Sphere {
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

                return Some(HitRecord::new(t, point, normal))
            }

            // try larger t
            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let point = r.point_at_param(t);
                let normal = (r.point_at_param(t) - &self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal))
            }
        }

        return None;
    }
}
