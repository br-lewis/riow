
use rand::Rng;

use crate::{Vec3, Ray};
use crate::hit::HitRecord;
use super::{Material, reflect};

pub struct Dielectric {
    ref_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric {
            ref_index: refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray_in.direction(), &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(&ray_in.direction(), &rec.normal) > 0.0 {
            let cosine = self.ref_index * Vec3::dot(&ray_in.direction(), &rec.normal) / ray_in.direction().len();
            (-&rec.normal, self.ref_index, cosine)
        } else {
            let cosine = -Vec3::dot(&ray_in.direction(), &rec.normal) / ray_in.direction().len();
            (rec.normal.clone(), 1.0/self.ref_index, cosine)
        };

        match refract(&ray_in.direction(), &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_threshold = schlick_approx(cosine, self.ref_index);
                let mut rng = rand::thread_rng();
                let reflect_prob: f64 = rng.gen();

                if reflect_prob < reflect_threshold {
                    let s = Ray::with_values(rec.point.clone(), reflected);
                    Some((s, attenuation))
                } else {
                    let s = Ray::with_values(rec.point.clone(), refracted);
                    Some((s, attenuation))
                }
            },
            None => {
                let s = Ray::with_values(rec.point.clone(), reflected);
                Some((s, attenuation))
            },
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0-dt*dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n*dt) - n*discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick_approx(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
