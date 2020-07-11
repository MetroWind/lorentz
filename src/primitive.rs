use std::vec::Vec;

use crate::vec3;
use vec3::Vec3;
use vec3::Color;
use crate::config::Float;
use crate::ray::{Ray,Hit};

pub trait Primitive
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>;
}

pub struct Sphere
{
    pub center: Vec3,
    pub radius: Float,
    pub color: Color,
    pub material: usize,
}

impl Primitive for Sphere
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>
    {
        let oc = r.origin - self.center;
        let a = r.dir.normSquared();
        let b = vec3::dot(&oc, &r.dir);
        let c = oc.normSquared() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0
        {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if !(temp < t_max && temp > t_min)
            {
                temp = (-b + discriminant.sqrt()) / a;
                if !(temp < t_max && temp > t_min)
                {
                    return None;
                }
            }

            let p = r.at(temp);
            return Some(Hit { t: temp, p: p,
                              normal: (p - self.center) / self.radius,
                              material: self.material,
            });
        }
        return None;
    }
}

pub struct PrimitiveList
{
    pub primitives: Vec<Box<dyn Primitive>>,
}

impl Primitive for PrimitiveList
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>
    {
        let mut closest: Float = t_max;
        let mut the_hit: Option<Hit> = None;

        for prim in &self.primitives
        {
            if let Some(hit) = prim.intersect(r, t_min, closest)
            {
                closest = hit.t;
                the_hit = Some(hit);
            }
        }
        the_hit
    }
}
