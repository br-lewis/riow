
mod lambertian;
mod metal;

use rand::Rng;

use super::Ray;
use super::Vec3;
use super::hit::HitRecord;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if v.sq_len() >= 1.0 {
            return v;
        }
    }
}
