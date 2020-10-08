// -*- mode: c++; -*-
#ifndef LORENTZ_SCENE_H
#define LORENTZ_SCENE_H

#include <vector>
#include <memory>

#include "camera.h"
#include "primitive.h"
#include "camera.h"
#include "material.h"

namespace lorentz
{
    struct Scene
    {
        Scene() = default;
        Scene(Scene&&) = default;

        uint32_t width;
        uint32_t height;
        Camera camera;
        PrimitiveList primitives;
        std::vector<MaterialPtr> materials;
    };


} // namespace lorentz

#endif
