#ifndef LORENTZ_CAMERA_H
#define LORENTZ_CAMERA_H

#include "vec3.h"
#include "ray.h"

namespace lorentz
{
    class Camera
    {
    public:
        Vec3 origin;

        Camera(const Vec3& look_from, const Vec3& look_at, const Vec3& vup,
               const Float vfov, const Float aspect, const Float aperture,
               const Float focus_distance);

        Ray ray(const Float s, const Float t) const;

    private:
        Vec3 lower_left;
        Vec3 hori, vert;
        Float lens_radius;
        Vec3 u, v, _w;
    };

} // namespace lorentz


#endif
