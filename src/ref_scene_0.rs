use std::sync::Arc;

use rand;

use crate::config::Float;
use crate::geometry;
use crate::geometry::Vec3;
use crate::geometry::BoundedPrimitive;
use crate::geometry::PrimitiveList;
use crate::material;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::geometry::BvhNode;

fn randomSphere() -> primitive::Sphere
{
    let x: Float = rand::random::<Float>() * 10.0 - 5.0;
    let z: Float = rand::random::<Float>() * 10.0 - 6.0;
    let r = rand::random::<Float>() * 0.02 + 0.09;
    primitive::Sphere {
        center: Vec3::new(x, -0.5 + r, z),
        radius: r,
        material: (rand::random::<Float>() * 17.0) as usize,
    }
}

fn randomSmallSphere() -> primitive::Sphere
{
    let x: Float = rand::random::<Float>() * 10.0 - 5.0;
    let z: Float = rand::random::<Float>() * 10.0 - 6.0;
    let r = rand::random::<Float>() * 0.005 + 0.02;
    primitive::Sphere {
        center: Vec3::new(x, -0.5 + r, z),
        radius: r,
        material: (rand::random::<Float>() * 17.0) as usize,
    }
}

fn buildPrimitives() -> PrimitiveList
{
    let mut stuff: Vec<Arc<dyn BoundedPrimitive + Sync + Send>> = vec![
         Arc::new(primitive::Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: 0,
        }),
        Arc::new(primitive::Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: 3,
        }),
        Arc::new(primitive::Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: 1,
        }),
    ];

    for _ in 0..300
    {
        stuff.push(Arc::new(randomSphere()));
    }

    PrimitiveList::new(stuff, vec![
        Arc::new(primitive::InfinitePlane {
            origin: Vec3::new(0.0, -0.5, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: 2,
        }),])
}

pub fn buildScene(width: u32, height: u32) -> Scene
{
    let camera_pos = Vec3::new(3.5, 0.35, 1.0);
    let camera_lookat = Vec3::new(0.0, -0.4, -1.0);

    let mut mats: Vec<Arc<dyn material::Material + Sync + Send>> = vec![
        Arc::new(material::Metal { albedo: Vec3::new(0.5, 0.5, 0.5), roughness: 0.0 }),
        Arc::new(material::Lambertian { albedo: Vec3::new(0.7, 0.7, 0.2) }),
        Arc::new(material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) }),
        Arc::new(material::Glass { ref_index: 1.5 }),
        Arc::new(material::Glass { ref_index: 1.7 }),
        Arc::new(material::Glass { ref_index: 1.7 }),
        Arc::new(material::Metal { albedo: Vec3::new(0.4, 0.5, 0.6), roughness: 0.1 }),
    ];
    for _ in 0..10
    {
        mats.push(Arc::new(material::LambertianRandomColor::new()));
    }

    Scene {
        width: width, height: height,
        camera: Camera::new(
            camera_pos, camera_lookat, Vec3::new(0.0, 1.0, 0.0), 40.0,
            width as Float / height as Float,
            0.06, (camera_lookat - camera_pos).norm() - 0.5,),
        primitives: buildPrimitives(),
        materials: mats,
    }
}
