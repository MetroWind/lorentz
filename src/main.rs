#![allow(non_snake_case)]

extern crate image;
extern crate rand;
use image::RgbImage;

mod config;
mod vec3;
mod tracer;
mod ray;
mod primitive;
mod material;

fn main()
{
    let img: RgbImage = tracer::render();
    if img.save_with_format("test.png", image::ImageFormat::Png).is_err()
    {
        panic!("Failed to generate image");
    }
}
