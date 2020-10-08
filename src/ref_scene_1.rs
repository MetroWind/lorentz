use std::sync::Arc;

use crate::config::Float;
use crate::geometry::Vec3;
use crate::geometry;
use crate::geometry::BoundedPrimitive;
use crate::geometry::PrimitiveList;
use crate::material;
use crate::scene::Camera;
use crate::scene::Scene;

fn buildPrimitives() -> PrimitiveList
{
    let mut stuff: Vec<Arc<dyn BoundedPrimitive + Sync + Send>> = vec![
         Arc::new(geometry::Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: 0,
        }),
        Arc::new(geometry::Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: 3,
        }),
        Arc::new(geometry::Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: 1,
        }),
    ];

    for i in 0..20
    {
        let x = -5.0 + 0.5 * (i as Float);
        for j in 0..20
        {
            let z = -5.0 + 0.5 * (j as Float);
            stuff.push(Arc::new(geometry::Sphere {
                center: Vec3::new(x, -0.4, z),
                radius: 0.1,
                material: (i * 20 + j) % 17,
            }));
        }
    }

    PrimitiveList::new(stuff, vec![
        Arc::new(geometry::InfinitePlane {
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
