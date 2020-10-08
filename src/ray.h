// -*- mode: c++ -*-
#ifndef LORENTZ_RAY_H
#define LORENTZ_RAY_H

#include "config.h"
#include "vec3.h"

namespace lorentz
{
    struct Ray
    {
        Vec3 origin;
        Vec3 dir;

        Vec3 at(Float t) const { return origin + dir * t; }

    };

    class Material;

    struct Hit
    {
        Float t;
        Vec3 p;
        Vec3 normal;
        Material* material;
    };

} // namespace lorentz


#endif
