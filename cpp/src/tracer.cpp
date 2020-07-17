#include <iostream>
#include <fstream>

#include "tracer.h"
#include "ref_scene_1.h"
#include "tile.h"

namespace lorentz
{
    Color renderRay(const Ray& r, const Scene& scene, uint32_t count)
    {
        if(count > 32)
        {
            return Color::origin();
        }

        // Set min hit distance to some small number to address the
        // surface acne problem.
        if(auto hit = scene.primitives.intersect(r, 0.0001f, 1000.0f);
           hit.has_value())
        {
            if(auto maybe_scattered = hit -> material -> scatter(r, *hit);
               maybe_scattered.has_value())
            {
                auto& [scattered, att] = *maybe_scattered;
                return att * renderRay(scattered, scene, count+1);
            }
            else
            {
                return Color::origin();
            }
        }

        // Background
        auto t = (Vec3::unit(r.dir)[1] + 1.0f) * 0.5f;
        auto c = (1.0f - t) * Vec3(1.0f, 1.0f, 1.0f) + t * Vec3(0.5f, 0.7f, 1.0f);
        return c;
    }

    Color renderPixel(const Scene& scene, uint32_t ns, uint32_t x, uint32_t y)
    {
        Color col = Color::origin();
        for(uint32_t i = 0; i < ns; i++)
        {
            Float u = (Float(x) + random()) / Float(scene.width);
            Float v = (Float(scene.height - y - 1) + random()) /
                Float(scene.height);

            auto r = scene.camera.ray(u, v);
            col += renderRay(r, scene, 0);
        }

        col /= Float(ns);

        // Gamma correction. For now we’ll assume gamma = 2.2.
        Float gamma = 1.0/2.2;
        col[0] = std::pow(col[0], gamma);
        col[1] = std::pow(col[1], gamma);
        col[2] = std::pow(col[2], gamma);

        return col;
    }

    void renderTile(const Scene& scene, uint32_t ns, CanvasTile& tile)
    {
        tile.img.resize((tile.xrange[1] - tile.xrange[0]) *
                        (tile.yrange[1] - tile.yrange[0]) * 3);
        size_t i = 0;
        for(uint32_t y = tile.yrange[0]; y < tile.yrange[1]; y++)
        {
            for(uint32_t x = tile.xrange[0]; x < tile.xrange[1]; x++)
            {
                const Color c = renderPixel(scene, ns, x, y);
                tile.img[i++] = uint8_t(c[0] * COLOR_MAX);
                tile.img[i++] = uint8_t(c[1] * COLOR_MAX);
                tile.img[i++] = uint8_t(c[2] * COLOR_MAX);
            }
        }
    }

    void render(const std::string& filename)
    {
        const uint32_t width = 800;
        const uint32_t height = 500;
        const uint32_t tile_size = 64;
        auto scene = ref_scene_1::buildScene(width, height);

        // Signal to noise ratio, in some arbitrary scale.
        uint32_t snr_index = 2;
        // Number of samples per pixel.
        uint32_t ns = snr_index * snr_index;

        std::vector<uint8_t> img(width * height * 3);
        TiledCanvas canvas(width, height, tile_size);

        while(true)
        {
            auto tile = canvas.nextTile();
            if(!tile)
            {
                break;
            }
            std::cout << "Rendering " << tile->tile_idx[0] << ", "
                      << tile->tile_idx[1] << std::endl;
            renderTile(scene, ns, *tile);
            size_t i = 0;
            for(uint32_t y = tile->yrange[0]; y < tile->yrange[1]; y++)
            {
                for(uint32_t x = tile->xrange[0]; x < tile->xrange[1]; x++)
                {
                    const size_t img_idx = (y * width + x) * 3;
                    img[img_idx + 0] = tile->img[i + 0];
                    img[img_idx + 1] = tile->img[i + 1];
                    img[img_idx + 2] = tile->img[i + 2];
                    i += 3;
                }
            }
        }

        auto f = std::ofstream(filename.c_str());
        f << "P3\n" << width << " " << height << "\n255\n";
        for(uint32_t y = 0; y < height; y++)
        {
            for(uint32_t x = 0; x < width; x++)
            {
                f << int(img[(y * width + x) * 3]) << " "
                  << int(img[(y * width + x) * 3 + 1]) << " "
                  << int(img[(y * width + x) * 3 + 2]) << "\n";
            }
        }
        f.close();
    }


} // namespace lorentz
