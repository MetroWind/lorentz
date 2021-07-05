use crate::config::Float;
use crate::geometry::{Vec3, Color};

pub trait Texture
{
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Color;
}

pub struct Constant
{
    color: Color,
}

impl Constant
{
    pub fn new(c: Color) -> Self
    {
        Self { color: c }
    }
}

impl Texture for Constant
{
    fn value(&self, _: Float, _: Float, _: &Vec3) -> Color
    {
        self.color
    }
}
