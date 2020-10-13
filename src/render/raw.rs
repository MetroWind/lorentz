use std::ops::{Index, IndexMut};

use crate::geometry::Color;
use crate::config::Float;

const DUMB_COLOR_MAX: Float = 255.999;

/// An image with float channels. The this the image type that is
/// generated directly from the renderer. It should be tonemapped
/// before viewing.
pub struct RawImage
{
    data: Vec<Color>,           // x is inner loop.
    width: u32,
    height: u32,
}

impl RawImage
{
    /// Constructed a blank image with a size.
    pub fn new(width: u32, height: u32) -> Self
    {
        Self {
            data: vec![Color::origin(); (width * height) as usize],
            width: width,
            height: height
        }
    }

    /// Width of the image
    pub const fn width(&self) -> u32
    {
        self.width
    }

    /// Height of the image
    pub const fn height(&self) -> u32
    {
        self.height
    }

    const fn xy2Index(&self, x: u32, y: u32) -> usize
    {
        (y * self.width + x) as usize
    }

    /// Return a `RawImageView` of the image, given a “viewport”.
    pub fn view(&mut self, x0: u32, y0: u32, width: u32, height: u32) ->
        RawImageView
    {
        RawImageView::new(self, x0, y0, width, height)
    }

    /// Tonemap the image to produce a “regular” 24 bit image in the
    /// form of a vector. The loop order from outter to inner is row →
    /// column → channel (x changes faster than y), where channel
    /// order is RGB.
    pub fn tonemap(self) -> Vec<u8>
    {
        let mut result: Vec<u8> = vec![0; (self.width * self.height * 3) as usize];
        let mut i: usize = 0;
        for mut c in self.data
        {
            // Gamma correction. For now we’ll assume gamma = 2.2.
            let g_correct: Float = 1.0/2.2;
            c[0] = c[0].powf(g_correct);
            c[1] = c[1].powf(g_correct);
            c[2] = c[2].powf(g_correct);

            // Tone map to u8.
            result[i] = (c[0] * DUMB_COLOR_MAX) as u8;
            result[i+1] = (c[1] * DUMB_COLOR_MAX) as u8;
            result[i+2] = (c[2] * DUMB_COLOR_MAX) as u8;
            i += 3;
        }
        result
    }
}

impl Index<(u32, u32)> for RawImage
{
    type Output = Color;
    fn index(&self, coord: (u32, u32)) -> &Self::Output
    {
        &self.data[self.xy2Index(coord.0, coord.1)]
    }
}

impl IndexMut<(u32, u32)> for RawImage
{
    fn index_mut(&mut self, coord: (u32, u32)) -> &mut Self::Output
    {
        let index = self.xy2Index(coord.0, coord.1);
        &mut self.data[index]
    }
}

/// A non-owning view of a portion of an image. This is the equivalent
/// of a slice for an image.
#[derive(Clone)]
pub struct RawImageView
{
    data: *mut Color,
    orig_width: u32,
    orig_height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
}

unsafe impl Send for RawImageView {}
unsafe impl Sync for RawImageView {}

impl RawImageView
{
    /// Construct a view into a rectangle region of `img`. The
    /// rectangle is defined by a top-left (or bottom-left, depending
    /// on your definition of y-direction) corner and a size.
    pub fn new(img: &mut RawImage, x0: u32, y0: u32, width: u32, height: u32) ->
        Self
    {
        Self{ data: img.data.as_mut_ptr(),
              orig_width: img.width(), orig_height: img.height(),
              offset_x: x0, offset_y: y0,
              width: width, height: height, }
    }

    const fn xy2Index(&self, x: u32, y: u32) -> usize
    {
        ((y + self.offset_y) * self.orig_width + x + self.offset_x) as usize
    }

    /// Set the pixel at (`x`, `y`) in the view to `color`.
    pub fn set(&mut self, x: u32, y: u32, color: Color)
    {
        let index = self.xy2Index(x, y);
        unsafe {
            self.data.add(index).write(color);
        }
    }
}

impl Index<(u32, u32)> for RawImageView
{
    type Output = Color;
    fn index(&self, coord: (u32, u32)) -> &Self::Output
    {
        unsafe {
            self.data.add(self.xy2Index(coord.0, coord.1)).as_ref().unwrap()
        }
    }
}
