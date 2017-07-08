use std::ops::{Add, Sub, Mul, Neg};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3::from_one(0.0)
    }

    pub fn from_one(v: f32) -> Vec3 {
        Vec3 { x: v, y: v, z: v }
    }

    pub fn length(&self) -> f32 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let s1 = (self.y*other.z) - (self.z*other.y);
        let s2 = (self.z*other.x) - (self.x*other.z);
        let s3 = (self.x*other.y) - (self.y*other.x);
        Vec3 { x: s1, y: s2, z: s3 }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}
