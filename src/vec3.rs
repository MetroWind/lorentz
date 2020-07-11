use std::ops;

use rand;

use crate::config::Float;

#[derive(Clone, Copy, Debug)]
pub struct Vec3
{
    pub data: [Float; 3],
}

pub type Color = Vec3;

impl ops::Add for Vec3
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        Self { data: [self.data[0] + other.data[0],
                      self.data[1] + other.data[1],
                      self.data[2] + other.data[2],] }
    }
}

impl ops::AddAssign for Vec3
{
    fn add_assign(&mut self, other: Self)
    {
        *self = Self { data: [self.data[0] + other.data[0],
                              self.data[1] + other.data[1],
                              self.data[2] + other.data[2],]};
    }
}

impl ops::Sub for Vec3
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        Self { data: [self.data[0] - other.data[0],
                      self.data[1] - other.data[1],
                      self.data[2] - other.data[2],] }
    }
}

impl ops::SubAssign for Vec3
{
    fn sub_assign(&mut self, other: Self)
    {
        *self = Self { data: [self.data[0] - other.data[0],
                              self.data[1] - other.data[1],
                              self.data[2] - other.data[2],]};
    }
}

impl ops::Mul<Float> for Vec3
{
    type Output = Self;
    fn mul(self, rhs: Float) -> Self
    {
        Self { data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs] }
    }
}

impl ops::Mul<Vec3> for Float
{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3
    {
        Vec3 { data: [rhs.data[0] * self, rhs.data[1] * self, rhs.data[2] * self] }
    }
}

impl ops::Mul for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3
    {
        Self::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::MulAssign for Vec3
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = Self::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]);
    }
}

impl ops::MulAssign<Float> for Vec3
{
    fn mul_assign(&mut self, rhs: Float)
    {
        *self = Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs);
    }
}

impl ops::Div<Float> for Vec3
{
    type Output = Self;
    fn div(self, rhs: Float) -> Self
    {
        let scale: Float = 1.0 / rhs;
        self * scale
    }
}

impl ops::Div for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3
    {
        Self::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl ops::DivAssign for Vec3
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = Self::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2]);
    }
}

impl ops::DivAssign<Float> for Vec3
{
    fn div_assign(&mut self, rhs: Float)
    {
        let scale: Float = 1.0 / rhs;
        *self = Self::new(self[0] * scale, self[1] * scale, self[2] * scale);
    }
}

impl ops::Neg for Vec3
{
    type Output = Self;
    fn neg(self) -> Self
    {
        return -1.0 * self;
    }
}

impl Vec3
{
    pub fn new(x: Float, y: Float, z: Float) -> Vec3
    {
        Vec3 { data: [x, y, z] }
    }

    pub fn unit(v: &Vec3) -> Vec3
    {
        *v / v.norm()
    }

    pub fn origin() -> Vec3
    {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn norm(&self) -> Float
    {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn normSquared(&self) -> Float
    {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn randInUnitSphere() -> Vec3
    {
        let mut p: Vec3;
        loop
        {
            p = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random()) -
                Vec3::new(1.0, 1.0, 1.0);
            if p.normSquared() < 1.0
            {
                return p;
            }
        }
    }
}

impl ops::Index<usize> for Vec3
{
    type Output = Float;

    fn index(&self, i: usize) -> &Self::Output
    {
        &self.data[i]
    }
}

impl ops::IndexMut<usize> for Vec3
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output
    {
        &mut self.data[i]
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> Float
{
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3
{
    Vec3::new(lhs[1] * rhs[2] - lhs[2] * rhs[1],
              lhs[2] * rhs[0] - lhs[0] * rhs[2],
              lhs[0] * rhs[1] - lhs[1] * rhs[0])
}
