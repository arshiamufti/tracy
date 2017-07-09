use vector::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.a + t*self.b
    }
}
