
#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div, Index,
    AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3{
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn with_values(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3{
            e: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn sq_len(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.len();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(l: &Self, r: &Self) -> f64 {
        l.e[0] * r.e[0] + l.e[1] * r.e[1] + l.e[2] * r.e[2]
    }

    pub fn cross(l: &Self, r: &Self) -> Self {
        Vec3::with_values(
            l.e[1] * r.e[2] - l.e[2] * r.e[1],
            -(l.e[0] * r.e[2] - l.e[2] * r.e[0]),
            l.e[0] * r.e[1] - l.e[1] * r.e[0]
        )
    }

    pub fn unit_vector(v: &Self) -> Self {
        v / v.len()
    }
}

// TODO: there must be a better way to handle this than to make an impl for each variation
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Div<&Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: &Vec3) -> Vec3 {
        Vec3::with_values(
            self.e[0] / o.e[0],
            self.e[1] / o.e[1],
            self.e[2] / o.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::with_values(
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
        )
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, o: &Vec3) -> Vec3 {
        Vec3::with_values(
            self * o.e[0],
            self * o.e[1],
            self * o.e[2],
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, o: Vec3) -> Vec3 {
        Vec3::with_values(
            self * o.e[0],
            self * o.e[1],
            self * o.e[2],
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::with_values(
            self.e[0] / t,
            self.e[1] / t,
            self.e[2] / t,
        )
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::with_values(
            self.e[0] / t,
            self.e[1] / t,
            self.e[2] / t,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Self) {
        self.e[0] += o.e[0];
        self.e[1] += o.e[1];
        self.e[2] += o.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, o: Self) {
        self.e[0] *= o.e[0];
        self.e[1] *= o.e[1];
        self.e[2] *= o.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, o: Self) {
        self.e[0] /= o.e[0];
        self.e[1] /= o.e[1];
        self.e[2] /= o.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Self) {
        self.e[0] -= o.e[0];
        self.e[1] -= o.e[1];
        self.e[2] -= o.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, o: f64) {
        self.e[0] *= o;
        self.e[1] *= o;
        self.e[2] *= o;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, o: f64) {
        self.e[0] /= o;
        self.e[1] /= o;
        self.e[2] /= o;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        /*
        if i > self.e.len() {
            panic!("out of bounds access");
        }
        */
        &self.e[i]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
