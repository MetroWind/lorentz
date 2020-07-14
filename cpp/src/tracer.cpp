#include <iostream>
#include <fstream>

#include "tracer.h"
#include "ref_scene_1.h"

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

    void render(const std::string& filename)
    {
        uint32_t width = 800;
        uint32_t height = 500;
        auto scene = ref_scene_1::buildScene(width, height);

        // Signal to noise ratio, in some arbitrary scale.
        uint32_t snr_index = 3;
        // Number of samples per pixel.
        uint32_t ns = snr_index * snr_index;

        auto f = std::ofstream(filename.c_str());
        f << "P3\n" << width << " " << height << "\n255\n";
        for(uint32_t y = 0; y < height; y++)
        {
            for(uint32_t x = 0; x < width; x++)
            {
                Color p = renderPixel(scene, ns, x, y);
                f << int(p[0] * COLOR_MAX) << " "
                  << int(p[1] * COLOR_MAX) << " "
                  << int(p[2] * COLOR_MAX) << "\n";
            }
        }
        f.close();
    }


} // namespace lorentz
