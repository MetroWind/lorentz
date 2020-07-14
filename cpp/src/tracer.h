// -*- mode: c++; -*-
#ifndef LORENTZ_TRACER_H
#define LORENTZ_TRACER_H

#include "ray.h"
#include "scene.h"
#include "ref_scene_1.h"

namespace lorentz
{
    constexpr Float COLOR_MAX = 255.999;

    Color renderRay(const Ray& r, const Scene& scene, uint32_t count);
    Color renderPixel(const Scene& scene, uint32_t ns, uint32_t x, uint32_t y);

    void render(const std::string& filename);

} // namespace lorentz

#endif
