#![allow(non_snake_case)]

extern crate image;
extern crate rand;
extern crate rayon;

mod config;
mod geometry;
mod texture;
mod material;
mod scene;
mod render;
mod ref_scene_1;

fn main()
{
    let img = render::render();
    let width = img.width();
    let height = img.height();

    if image::save_buffer_with_format(
        "test.png", &img.tonemap(), width, height,
        image::ColorType::Rgb8, image::ImageFormat::Png).is_err()
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
