use crate::vec3::Vec3;
use crate::config::Float;

// origin + dir * t
pub struct Ray
{
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray
{
    pub fn at(&self, t: Float) -> Vec3
    {
        self.origin + self.dir * t
    }
}

pub struct Hit
{
    pub t: Float,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: usize,
}
