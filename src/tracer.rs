use image::{ImageBuffer, RgbImage};
use rand::prelude::*;

use crate::vec3::{Vec3, Color};
use crate::config::Float;
use crate::ray::Ray;
use crate::primitive;
use crate::primitive::{Primitive,PrimitiveList};
use crate::material;

static COLOR_MAX: Float = 255.999;

pub struct Camera
{
    lower_left: Vec3,
    hori: Vec3,
    vert: Vec3,
    pub origin: Vec3,
}

impl Camera
{
    pub fn new() -> Self
    {
        Self { lower_left: Vec3::new(-2.0, -1.0, -1.0),
               hori: Vec3::new(4.0, 0.0, 0.0),
               vert: Vec3::new(0.0, 2.0, 0.0),
               origin: Vec3::origin(),
        }
    }

    pub fn ray(&self, u: Float, v: Float) -> Ray
    {
         Ray { origin: self.origin,
               dir: self.lower_left + u * self.hori + v * self.vert
               - self.origin }
    }
}

pub struct Scene
{
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub primitives: PrimitiveList,
    pub materials: Vec<Box<material::Material>>,
}

// fn renderRay(r: &Ray, scene: &Scene) -> Color
// {
//     if let Some(hit) = scene.primitives.intersect(r, 0.0, 10.0)
//     {
//         return (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
//     }

//     // Background
//     let t = (Vec3::unit(&r.dir)[1] + 1.0) * 0.5;
//     let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
//     c
// }

fn renderRay(r: &Ray, scene: &Scene, count: u32) -> Color
{
    if count > 32
    {
        return Color::origin();
    }

    // Set min hit distance to some small number to address the
    // surface acne problem.
    if let Some(hit) = scene.primitives.intersect(r, 0.0001, 1000.0)
    {
        if let Some((scattered, att)) = scene.materials[hit.material].scatter(r, &hit)
        {
            return att * renderRay(&scattered, scene, count+1);
        }
        else
        {
            return Color::origin();
        }
    }

    // Background
    let t = (Vec3::unit(&r.dir)[1] + 1.0) * 0.5;
    let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    c
}

fn buildPrimitives() -> PrimitiveList
{
    PrimitiveList {
        primitives: vec![
            Box::new(primitive::Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                color: Color::new(0.0, 0.0, 0.0),
                material: 0,
            }),
            Box::new(primitive::Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                color: Color::new(0.0, 0.0, 0.0),
                material: 2,
            }),
            Box::new(primitive::Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                color: Color::new(0.0, 0.0, 0.0),
                material: 1,
            }),
            Box::new(primitive::Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                color: Color::new(0.0, 0.0, 0.0),
                material: 1,
            }),
        ]}
}

pub fn render() -> RgbImage
{
    let mats: Vec<Box<material::Material>> = vec![
        Box::new(material::Metal { albedo: Vec3::new(0.95, 0.5, 0.5), roughness: 0.0 }),
        Box::new(material::Lambertian { albedo: Vec3::new(0.7, 0.7, 0.2) }),
        Box::new(material::Glass { ref_index: 1.5 }),
    ];
    let scene = Scene {
        width: 800, height: 400, camera: Camera::new(),
        primitives: buildPrimitives(),
        materials: mats,
    };

    // Signal to noise ratio, in some arbitrary scale.
    let snr_index: u32 = 16;
    // Number of samples per pixel.
    let ns = snr_index * snr_index;

    ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
        let mut col = Color::origin();

        for _ in 0..ns
        {
            let u: Float = (x as Float + rand::random::<Float>()) / scene.width as Float;
            let v: Float = ((scene.height - y - 1) as Float + rand::random::<Float>()) /
                scene.height as Float;

            let r = scene.camera.ray(u, v);
            col += renderRay(&r, &scene, 0);
        }
        col /= ns as Float;

        // Gamma correction. For now we’ll assume gamma = 2.2.
        let g_correct: Float = 1.0/2.2;
        col[0] = col[0].powf(g_correct);
        col[1] = col[1].powf(g_correct);
        col[2] = col[2].powf(g_correct);

        image::Rgb([(col[0] * COLOR_MAX) as u8,
                    (col[1] * COLOR_MAX) as u8,
                    (col[2] * COLOR_MAX) as u8])
    })
}
