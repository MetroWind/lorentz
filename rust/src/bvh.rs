use std::sync::Arc;

use rand;

use crate::config::Float;
use crate::ray::{Ray, Hit};
use crate::primitive_traits::{Primitive, BBox, BoundedPrimitive};

// pub static mut node_inter_count_true: usize = 0;
// pub static mut node_inter_count_false: usize = 0;
// pub static mut obj_inter_count: usize = 0;

struct BvhBranch
{
    left: Arc<BvhNode>,
    right: Arc<BvhNode>,
}

struct BvhLeaf
{
    obj: Arc<dyn BoundedPrimitive + Send + Sync>,
}

enum BvhData
{
    Branch(BvhBranch),
    Leaf(BvhLeaf),
}

pub struct BvhNode
{
    data: BvhData,
    pub bounding_box: BBox,
}

impl BvhNode
{
    pub fn new(prims: &mut [Arc<dyn BoundedPrimitive + Send + Sync>]) -> Self
    {
        let len = prims.len();
        let ax = (rand::random::<Float>() * 3.0) as usize;
        if prims.len() == 1
        {
            // println!("Bounding box for obj is {:?}", prims[0].bbox());
            Self {
                data: BvhData::Leaf(BvhLeaf {obj: prims[0].clone()}),
                bounding_box: prims[0].bbox(),
            }
        }
        else
        {
            prims.sort_by(
                |a, b| { a.bbox().lower[ax].partial_cmp(&b.bbox().lower[ax]).unwrap() });
            let left = Arc::new(Self::new(&mut prims[0..len / 2]));
            let right = Arc::new(Self::new(&mut prims[len / 2..len]));
            // println!("Bounding box for node is \n\t{:?} +\n\t{:?} -->\n\t {:?}",
            //          left.bbox(), right.bbox(),
            //          left.bbox().union(&right.bbox()));
            Self {
                bounding_box: left.bbox().union(&right.bbox()),
                data: BvhData::Branch(BvhBranch { left: left, right: right }),
            }
        }
    }

}

impl Primitive for BvhNode
{
    fn intersect(&self, r: &Ray, t_min: Float, t_max: Float) -> Option<Hit>
    {
        if !self.bounding_box.hit(r, t_min, t_max)
        {
            // unsafe
            // {
            //     node_inter_count_false += 1;
            // }
            return None;
        }
        // unsafe
        // {
        //     node_inter_count_true += 1;
        // }
        match &self.data
        {
            BvhData::Leaf(leaf) =>
            {
                // unsafe
                // {
                //     obj_inter_count += 1;
                // }
                leaf.obj.intersect(r, t_min, t_max)
            },
            BvhData::Branch(branch) =>
            {
                let left_hit_maybe = branch.left.intersect(r, t_min, t_max);
                let right_hit_maybe = branch.right.intersect(r, t_min, t_max);

                if left_hit_maybe.is_some() && right_hit_maybe.is_some()
                {
                    let left_hit = left_hit_maybe.unwrap();
                    let right_hit = right_hit_maybe.unwrap();
                    if left_hit.t < right_hit.t
                    {
                        Some(left_hit)
                    }
                    else
                    {
                        Some(right_hit)
                    }
                }
                else if left_hit_maybe.is_some()
                {
                    left_hit_maybe
                }
                else if right_hit_maybe.is_some()
                {
                    right_hit_maybe
                }
                else
                {
                    None
                }
            },
        }
    }
}

impl BoundedPrimitive for BvhNode
{
    fn bbox(&self) -> BBox
    {
        self.bounding_box.clone()
    }
}
