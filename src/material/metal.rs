
use crate::hit::HitRecord;
use crate::Ray;
use crate::Vec3;
use super::Material;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(a: Vec3) -> Self {
        Self {
            albedo: a
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&Vec3::unit_vector(ray_in.direction()), &rec.normal);
        let scattered = Ray::with_values(rec.point, reflected);

        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, normal) * normal
}
