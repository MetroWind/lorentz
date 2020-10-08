use std::vec::Vec;
use std::sync::{Arc, Mutex};

use image::*;
use rayon;

use crate::geometry::{Vec3, Color};
use crate::config::Float;
use crate::geometry::Ray;
use crate::scene::Scene;
use crate::geometry::Primitive;
use super::tile::{TiledCanvas, CanvasTile};

use crate::ref_scene_1;

static COLOR_MAX: Float = 255.999;

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

fn renderTile(scene: &Scene, ns: u32, tile: &CanvasTile) -> CanvasTile
{
    let mut img = RgbImage::new(tile.xrange.1 - tile.xrange.0,
                                tile.yrange.1 - tile.yrange.0);
    for y in tile.yrange.0..tile.yrange.1
    {
        for x in tile.xrange.0..tile.xrange.1
        {
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

            img.put_pixel(
                x - tile.xrange.0, y - tile.yrange.0,
                Rgb::from_channels((col[0] * COLOR_MAX) as u8,
                                   (col[1] * COLOR_MAX) as u8,
                                   (col[2] * COLOR_MAX) as u8,
                                   0 as u8));
        }
    }
    let mut result: CanvasTile = tile.clone();
    result.img = Some(img);
    result
}

pub fn render() -> RgbImage
{
    let width = 800;
    let height = 500;
    let scene = Arc::new(ref_scene_1::buildScene(width, height));

    // Signal to noise ratio, in some arbitrary scale.
    let snr_index: u32 = 10;
    // Number of samples per pixel.
    let ns = snr_index * snr_index;

    let mut img: RgbImage = ImageBuffer::new(scene.width, scene.height);
    let canvas = TiledCanvas::new(scene.width, scene.height, 64);

    // Method 1: Rust std thread, doesn’t work...
    //
    // let canvas = Mutex::new(canvas);
    // let mut threads = vec![];
    //
    // for i in 0..8
    // {
    //     threads.push(thread::spawn(|| {
    //         loop
    //         {
    //             if let Some(tile) = canvas.lock().unwrap().nextTile()
    //             {
    //                 println!("Rendering tile {:?}...", tile.tile_idx);
    //                 let tile_img = renderTile(&scene, ns, &tile).img.unwrap();
    //                 for y in tile.yrange.0..tile.yrange.1
    //                 {
    //                     for x in tile.xrange.0..tile.xrange.1
    //                     {
    //                         img.put_pixel(
    //                             x, y, tile_img.get_pixel(
    //                                 x - tile.xrange.0, y - tile.yrange.0).clone());
    //                     }
    //                 }
    //             }
    //             else
    //             {
    //                 break;
    //             }
    //         }
    //     }));
    // }
    //
    // for t in threads
    // {
    //     t.join();
    // }

    // Method 2: Rayon’s scoped thread pool.
    //
    let tile_count_x = canvas.tile_count_x;
    let tile_count_y = canvas.tile_count_y;
    let canvas = Mutex::new(canvas);
    let tiles: Mutex<Vec<CanvasTile>> = Mutex::new(Vec::new());
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    pool.scope(|s| {
        for _ in 0..(tile_count_x * tile_count_y)
        {
            s.spawn(|_| {
                let tile = canvas.lock().unwrap().nextTile().unwrap();
                println!("Rendering tile {:?}...", tile.tile_idx);
                let tile_img = renderTile(&scene, ns, &tile);
                let mut tiles = tiles.lock().unwrap();
                tiles.push(tile_img);
            });
        }});

    for atile in tiles.into_inner().unwrap()
    {
        let tile_img = atile.img.unwrap();
        for y in atile.yrange.0..atile.yrange.1
        {
            for x in atile.xrange.0..atile.xrange.1
            {
                let pix = tile_img.get_pixel(
                    x - atile.xrange.0, y - atile.yrange.0).clone();
                img.put_pixel(x, y, pix);
            }
        }
    }

    // Method 3: Rayon’s parallel iterator. This for some reason
    // doesn’t render tiles in order.
    //
    // let tiles: Mutex<Vec<CanvasTile>> = Mutex::new(Vec::new());
    // (0..canvas.tile_count_x * canvas.tile_count_y).into_par_iter().for_each(
    //     |i| {
    //         if let Some(tile) = canvas.at(i)
    //         {
    //             println!("Rendering tile {:?}...", tile.tile_idx);
    //             // renderTile(&scene, ns, &tile, SubImage::<RgbImage>::new(img, 0, 0, 0, 0));
    //             let tile_img = renderTile(&scene, ns, &tile);
    //             let mut tiles = tiles.lock().unwrap();
    //             tiles.push(tile_img);
    //         }
    //     });
    //
    // for atile in tiles.into_inner().unwrap()
    // {
    //     let tile_img = atile.img.unwrap();
    //     for y in atile.yrange.0..atile.yrange.1
    //     {
    //         for x in atile.xrange.0..atile.xrange.1
    //         {
    //             let pix = tile_img.get_pixel(
    //                 x - atile.xrange.0, y - atile.yrange.0).clone();
    //             img.put_pixel(x, y, pix);
    //         }
    //     }
    // }

    img
}
