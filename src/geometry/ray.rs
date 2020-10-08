use super::vec3::Vec3;
use crate::config::Float;

/// A linear ray in the form of `origin` + `dir` * t, where t is the
/// “ray time”.
pub struct Ray
{
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray
{
    /// Calculate `origin` + `dir` * t at a given `t`.
    pub fn at(&self, t: Float) -> Vec3
    {
        self.origin + self.dir * t
    }
}

/// An abstraction of the point where a ray intersects with some
/// primitive.
pub struct Hit
{
    /// The ray time of the hit.
    pub t: Float,
    /// The coordinates of the intersection.
    pub p: Vec3,
    /// The normal vector of the geometry at the intersection.
    pub normal: Vec3,
    /// The material index at the intersection.
    pub material: usize,
}
