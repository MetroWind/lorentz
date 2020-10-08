#include "tile.h"

namespace lorentz
{
    TiledCanvas :: TiledCanvas(uint32_t w, uint32_t h, uint32_t ts)
            : tile_size(ts),
              tile_count_x(w % ts == 0 ? w/ts : w/ts + 1),
              tile_count_y(h % ts == 0 ? h/ts : h/ts + 1),
              width(w), height(h), next_tile({0, 0}) {}

    std::optional<CanvasTile> TiledCanvas :: nextTile()
    {
        if(next_tile[1] >= tile_count_y)
        {
            return std::nullopt;
        }

        CanvasTile tile;
        tile.tile_idx = next_tile;
        tile.xrange[0] = next_tile[0] * tile_size;
        tile.xrange[1] = (next_tile[0] + 1) * tile_size;
        if(tile.xrange[1] > width)
        {
            tile.xrange[1] = width;
        }

        tile.yrange[0] = next_tile[1] * tile_size;
        tile.yrange[1] = (next_tile[1] + 1) * tile_size;

        if(tile.yrange[1] > height)
        {
            tile.yrange[1] = height;
        }

        next_tile[0]++;
        if(next_tile[0] == tile_count_x)
        {
            next_tile[0] = 0;
            next_tile[1]++;
        }

        return tile;
    }



} // namespace lorentz
