use super::{RawImage, RawImageView};

pub struct TiledCanvas<'a>
{
    img: &'a mut RawImage,
    pub tile_size: u32,
    pub tile_count_x: u32,
    pub tile_count_y: u32,
}

impl<'a> TiledCanvas<'a>
{
    pub fn new(img: &'a mut RawImage, tile_size: u32) ->
        Self
    {
        let w = img.width();
        let h = img.height();
        Self {
            img: img,
            tile_size: tile_size,
            tile_count_x: if w % tile_size == 0
            { w / tile_size } else { w / tile_size + 1 },
            tile_count_y: if h % tile_size == 0
            { h / tile_size } else { h / tile_size + 1 },
        }
    }

    fn at(&mut self, i: u32) -> Option<RawImageView>
    {
        if i >= self.tile_count_x * self.tile_count_y
        {
            return None;
        }
        let tile_idx_y = i / self.tile_count_x;
        let tile_idx_x = i % self.tile_count_x;
        let x = tile_idx_x * self.tile_size;
        let mut width = self.tile_size;
        if x + width > self.img.width()
        {
            width = self.img.width() - x;
        }
        let y = tile_idx_y * self.tile_size;
        let mut height = self.tile_size;
        if y + height > self.img.height()
        {
            height = self.img.height() - y;
        }
        Some(self.img.view(x, y, width, height))
    }

    pub fn tiles(&mut self) -> Vec<RawImageView>
    {
        let mut result = Vec::new();
        let mut i = 0;
        loop
        {
            if let Some(tile) = self.at(i)
            {
                result.push(tile);
            }
            else
            {
                return result;
            }
            i += 1;
        }
    }
}
