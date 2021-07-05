use std::sync::Arc;

use crate::config::Float;
use crate::geometry::{Vec3, Color};

pub trait Texture
{
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Color;
}

pub type AnyTexture = Arc<dyn Texture + Sync + Send>;

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

pub struct Checker
{
    even: AnyTexture,
    odd: AnyTexture,
}

impl Checker
{
    pub fn new(even: AnyTexture, odd: AnyTexture) -> Self
    {
        Self { even: even, odd: odd }
    }
}

impl Texture for Checker
{
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Color
    {
        let sines: Float = (10.0 * p[0]).sin() * (10.0 * p[1]).sin() *
            (10.0 * p[2]).sin();
        if sines < 0.0
        {
            self.odd.value(u, v, p)
        }
        else
        {
            self.even.value(u, v, p)
        }
    }
}
