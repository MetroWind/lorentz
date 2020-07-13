use std::mem;

use crate::vec3;
use vec3::Vec3;
use crate::config::Float;
use crate::ray::{Ray,Hit};

#[derive(Clone, Debug)]
pub struct BBox
{
    pub lower: Vec3,
    pub higher: Vec3,
}

impl BBox
{
    pub fn hit(&self, r: &Ray, tmin: Float, tmax: Float) -> bool
    {
        for i in 0..3
        {
            let dir_inverse: Float = 1.0 / r.dir[i];
            let mut t0 = (self.lower[i] - r.origin[i]) * dir_inverse;
            let mut t1 = (self.higher[i] - r.origin[i]) * dir_inverse;
            if t0 > t1
            {
                mem::swap(&mut t0, &mut t1);
            }

            let the_tmin = if t0 > tmin {t0} else {tmin};
            let the_tmax = if t1 < tmax {t1} else {tmax};

            if the_tmax <= the_tmin { return false; }
        }
        true
    }

    pub fn union(&self, rhs: &BBox) -> BBox
    {
        BBox {
            lower: Vec3::new(
                if self.lower[0] < rhs.lower[0] {self.lower[0]} else {rhs.lower[0]},
                if self.lower[1] < rhs.lower[1] {self.lower[1]} else {rhs.lower[1]},
                if self.lower[2] < rhs.lower[2] {self.lower[2]} else {rhs.lower[2]}),
            higher: Vec3::new(
                if self.higher[0] < rhs.higher[0] {rhs.higher[0]} else {self.higher[0]},
                if self.higher[1] < rhs.higher[1] {rhs.higher[1]} else {self.higher[1]},
                if self.higher[2] < rhs.higher[2] {rhs.higher[2]} else {self.higher[2]})
        }
    }

}

pub trait Primitive
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>;
}

pub trait BoundedPrimitive: Primitive
{
    fn bbox(&self) -> BBox;
}
