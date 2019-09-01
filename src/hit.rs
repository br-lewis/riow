
//use std::rc::Rc;
use std::sync::Arc;

use super::{Ray, Vec3};
use super::material::Material;

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material + 'a + Sync>,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, point: Vec3, normal: Vec3, mat: Arc<dyn Material + Sync + 'a>) -> Self {
        Self {
            t: t,
            point: point,
            normal: normal,
            mat: mat,
        }
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitableList<'a>{
    hitables: Arc<Vec<Arc<dyn Hit + 'a>>>,
}

impl<'a> HitableList<'a> {
    pub fn new() -> Self {
        HitableList{
            hitables: Arc::new(Vec::new()),
        }
    }

    pub fn with_vals(hitables: Vec<Arc<dyn Hit>>) -> Self {
        HitableList {
            hitables: Arc::new(hitables),
        }
    }

    pub fn len(&self) -> usize {
        self.hitables.len()
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for h in self.hitables.iter() {
            match h.hit(r, t_min, t_max) {
                Some(record) => {
                    if let Some(c) = &closest {
                        if record.t < c.t {
                            closest = Some(record)
                        }
                    } else {
                        closest = Some(record)
                    }
                },
                None => {},
            };
        }

        closest
    }
}
