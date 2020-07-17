#ifndef LORENTZ_TILE_H
#define LORENTZ_TILE_H

#include <vector>
#include <array>
#include <optional>

namespace lorentz
{
    struct CanvasTile
    {
        std::array<uint32_t, 2> tile_idx;
        std::array<uint32_t, 2> xrange;
        std::array<uint32_t, 2> yrange;
        std::vector<uint8_t> img;
    };

    class TiledCanvas
    {
    public:
        const uint32_t tile_size;
        const uint32_t tile_count_x;
        const uint32_t tile_count_y;

        TiledCanvas(uint32_t w, uint32_t h, uint32_t ts);

        std::optional<CanvasTile> nextTile();

    private:
        const uint32_t width;
        const uint32_t height;
        std::array<uint32_t, 2> next_tile;
    };

} // lorentz

#endif
