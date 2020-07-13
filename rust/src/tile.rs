use image::RgbImage;

#[derive(Clone)]
pub struct CanvasTile
{
    pub tile_idx: (u32, u32),
    pub xrange: (u32, u32),
    pub yrange: (u32, u32),
    pub img: Option<RgbImage>,
}

pub struct TiledCanvas
{
    width: u32,
    height: u32,
    pub tile_size: u32,
    pub tile_count_x: u32,
    pub tile_count_y: u32,
    next_tile: u32,
}

impl TiledCanvas
{
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self
    {
        Self {
            width: width, height: height, tile_size: tile_size,
            tile_count_x: if width % tile_size == 0
            { width / tile_size } else { width / tile_size + 1 },
            tile_count_y: if height % tile_size == 0
            { height / tile_size } else { height / tile_size + 1 },
            next_tile: 0,
        }
    }

    pub fn at(&self, i: u32) -> Option<CanvasTile>
    {
        if i >= self.tile_count_x * self.tile_count_y
        {
            return None;
        }
        let tile_idx_y = i / self.tile_count_x;
        let tile_idx_x = i % self.tile_count_x;
        let mut xrange = (tile_idx_x * self.tile_size,
                          (tile_idx_x + 1) * self.tile_size);
        if xrange.1 > self.width
        {
            xrange.1 = self.width;
        }
        let mut yrange = (tile_idx_y * self.tile_size,
                          (tile_idx_y + 1) * self.tile_size);
        if yrange.1 > self.height
        {
            yrange.1 = self.height;
        }
        Some(CanvasTile{ tile_idx: (tile_idx_x, tile_idx_y),
                         xrange: xrange, yrange: yrange,
                         img: None,})
    }

    pub fn nextTile(&mut self) -> Option<CanvasTile>
    {
        let t_maybe = self.at(self.next_tile);
        if t_maybe.is_some()
        {
            self.next_tile += 1;
        }
        return t_maybe;
    }
}
