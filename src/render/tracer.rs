use std::sync::Arc;
use rayon;

use crate::geometry::{Vec3, Color};
use crate::config::Float;
use crate::geometry::Ray;
use crate::scene::Scene;
use crate::geometry::Primitive;
use super::raw::{RawImage, RawImageView};
use super::tile::TiledCanvas;

use crate::ref_scene_1;

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

fn renderTile(scene: &Scene, ns: u32, tile: &mut RawImageView)
{
    for dy in 0..tile.height
    {
        for dx in 0..tile.width
        {
            let mut col = Color::origin();
            let x = dx + tile.offset_x;
            let y = dy + tile.offset_y;

            for _ in 0..ns
            {
                let u: Float = (x as Float + rand::random::<Float>()) / scene.width as Float;
                let v: Float = ((scene.height - y - 1) as Float + rand::random::<Float>()) /
                    scene.height as Float;

                let r = scene.camera.ray(u, v);
                col += renderRay(&r, &scene, 0);
            }
            col /= ns as Float;
            tile.set(dx, dy, col);
        }
    }
}

pub fn render() -> RawImage
{
    let width = 800;
    let height = 500;
    let scene = Arc::new(ref_scene_1::buildScene(width, height));

    // Signal to noise ratio, in some arbitrary scale.
    let snr_index: u32 = 10;
    // Number of samples per pixel.
    let ns = snr_index * snr_index;

    let pool = rayon::ThreadPoolBuilder::new().num_threads(24).build().unwrap();
    let mut img = RawImage::new(width, height);
    let mut canvas = TiledCanvas::new(&mut img, 64);
    let tiles = canvas.tiles();
    pool.scope(|s| {
        let mut i = 0;
        for mut tile in tiles
        {
            let scene = scene.clone();
            s.spawn(move |_| {
                println!("Rendering tile {:?}...", i);
                renderTile(&scene, ns, &mut tile);
            });
            i += 1;
        }
    });
    img
}
