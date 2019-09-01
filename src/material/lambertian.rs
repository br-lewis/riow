
use crate::hit::HitRecord;
use crate::Ray;
use crate::Vec3;
use super::Material;
use super::random_in_unit_sphere;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self {
            albedo: a
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = &rec.point + &rec.normal + random_in_unit_sphere();
        let scattered = Ray::with_values(rec.point.clone(), target-rec.point.clone());

        Some((scattered, self.albedo.clone()))
    }
}

