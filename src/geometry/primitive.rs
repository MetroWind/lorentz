use std::vec::Vec;
use std::sync::Arc;

use super::vec3;
use vec3::Vec3;
use crate::config::Float;
use super::ray::{Ray,Hit};
use super::primitive_traits::{Primitive, BBox, BoundedPrimitive};
use super::bvh;

/// A sphere with a center and a radius.
#[derive(Clone, Copy, Debug)]
pub struct Sphere
{
    pub center: Vec3,
    pub radius: Float,
    pub material: usize,
}

// pub static mut prim_inter_count: usize = 0;

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

impl BoundedPrimitive for Sphere
{
    fn bbox(&self) -> BBox
    {
        BBox {
            lower: self.center -
                Vec3::new(self.radius, self.radius, self.radius),
            higher: self.center +
                Vec3::new(self.radius, self.radius, self.radius),
        }
    }
}

/// An infinite plane denoted by a point on the plane, and a normal
/// vector. Does not have a bounding box.
#[derive(Clone, Copy, Debug)]
pub struct InfinitePlane
{
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: usize,
}

impl Primitive for InfinitePlane
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>
    {
        let denomi = vec3::dot(&self.normal, &r.dir);
        if denomi == 0.0
        {
            // Ray is parallel to plane.
            return None;
        }
        let t = vec3::dot(&(self.origin - r.origin), &self.normal) / denomi;
        if t < t_max && t > t_min
        {
            return Some(Hit { t: t, p: r.at(t), normal: self.normal,
                              material: self.material });
        }
        else
        {
            return None;
        }
    }
}

/// A collection of primitives.
pub struct PrimitiveList
{
    bounded: Vec<Arc<dyn BoundedPrimitive + Send + Sync>>,
    unbounded: Vec<Arc<dyn Primitive + Send + Sync>>,
    bvh_tree: Arc<bvh::BvhNode>,
    use_bvh: bool,
}

impl PrimitiveList
{
    /// Construct from a collection of bounded primitives and a
    /// collection of unbounded primitives.
    pub fn new(mut bounded: Vec<Arc<dyn BoundedPrimitive + Send + Sync>>,
               unbounded: Vec<Arc<dyn Primitive + Send + Sync>>) -> Self
    {
        let tree = Arc::new(bvh::BvhNode::new(&mut bounded[..]));

        Self {
            bounded: bounded, unbounded: unbounded, bvh_tree: tree,
            use_bvh: false,
        }
    }
}

/// A `PrimitiveList` is also considered a primitive, in the sense
/// that it can be hit by a ray.
impl Primitive for PrimitiveList
{
    /// Return the hit by ray `r` at the nearest primitive (smallest
    /// ray time).
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>
    {
        let mut closest: Float = t_max;
        let mut the_hit: Option<Hit> = None;

        if self.use_bvh
        {
            if let Some(hit) = self.bvh_tree.intersect(r, t_min, closest)
            {
                closest = hit.t;
                the_hit = Some(hit);
            }
        }
        else
        {
            for prim in &self.bounded
            {
                // unsafe {
                //     prim_inter_count += 1;
                // }
                if let Some(hit) = prim.intersect(r, t_min, closest)
                {
                    closest = hit.t;
                    the_hit = Some(hit);
                }
            }
        }
        for prim in &self.unbounded
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

/// A `PrimitiveList` can also be consided bounded, if one only
/// considers the bounded primitives contained.
impl BoundedPrimitive for PrimitiveList
{
    /// Return the minimal bbox containing all the bounded primitives
    /// in the collection.
    fn bbox(&self) -> BBox
    {
        let mut b = self.bounded[0].bbox();
        for prim in &self.bounded
        {
            b = b.union(&prim.bbox());
        }
        b
    }
}
