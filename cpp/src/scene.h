#ifndef LORENTZ_SCENE_H
#define LORENTZ_SCENE_H

#include "ray.h"
#include "bvh.h"
#include "primitive_trait.h"
#include "camera.h"

namespace lorentz
{
    struct Scene
    {
        uint32_t width;
        uint32_t height;
        Camera camera;

    };


} // namespace lorentz

#endif
