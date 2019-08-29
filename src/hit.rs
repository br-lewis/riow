
use super::{Ray, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> Self {
        Self {
            t: t,
            point: point,
            normal: normal,
        }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList {
    hitables: Vec<Box<dyn Hit>>,
}

impl HitableList {
    pub fn new() -> Self {
        HitableList{
            hitables: Vec::new(),
        }
    }

    pub fn with_vals(hitables: Vec<Box<dyn Hit>>) -> Self {
        HitableList {
            hitables: hitables,
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
