#![allow(non_snake_case)]

extern crate image;
extern crate rand;
extern crate rayon;

use image::RgbImage;

mod config;
mod vec3;
mod tracer;
mod ray;
mod primitive;
mod material;
mod camera;
mod scene;
mod tile;
mod primitive_traits;
mod bvh;
mod ref_scene_1;

fn main()
{
    let img: RgbImage = tracer::render();
    if img.save_with_format("test.png", image::ImageFormat::Png).is_err()
    {
        panic!("Failed to generate image");
    }

    // unsafe {
    //     println!("node_inter_count_false: {}", bvh::node_inter_count_false);
    //     println!("node_inter_count_true: {}", bvh::node_inter_count_true);
    //     println!("obj_inter_count: {}", bvh::obj_inter_count);
    //     println!("prim_inter_count: {}", primitive::prim_inter_count);
    // }
}
