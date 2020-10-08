use crate::geometry::vec3;
use crate::geometry::Vec3;
use crate::config::{Float, PI};
use crate::geometry::Ray;

pub struct Camera
{
    lower_left: Vec3,
    hori: Vec3,
    vert: Vec3,
    pub origin: Vec3,
    lens_radius: Float,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
}

impl Camera
{
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3,
               vfov: Float, aspect: Float,
               aperture: Float, focus_distance: Float) -> Self
    {
        // Points away from target in film frame
        let w = Vec3::unit(&(look_from - look_at));
        // Points to the right in film frame
        let u = Vec3::unit(&vec3::cross(&vup, &w));
        // Points up in film frame
        let v = vec3::cross(&w, &u);

        let theta = vfov * PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;

        Self {
            lower_left: look_from - half_width * u * focus_distance -
                half_height * v * focus_distance - w * focus_distance,
            hori: 2.0 * half_width * u * focus_distance,
            vert: 2.0 * half_height * v * focus_distance,
            origin: look_from,
            lens_radius: aperture * 0.5,
            u: u, v: v, _w: w,
        }
    }

    pub fn ray(&self, s: Float, t: Float) -> Ray
    {
        let rd = self.lens_radius * Vec3::randInUnitDisk();
        let offset = self.u * rd[0] + self.v * rd[1];
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left + s * self.hori + t * self.vert
                - self.origin - offset }
    }
}
