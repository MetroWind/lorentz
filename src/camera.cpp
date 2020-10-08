#include "utils.h"
#include "camera.h"

namespace lorentz
{
    Camera :: Camera(const Vec3& look_from, const Vec3& look_at, const Vec3& vup,
                     const Float vfov, const Float aspect, const Float aperture,
                     const Float focus_distance)
    {
        // Points away from target in film frame
        auto ww = Vec3::unit(look_from - look_at);
        // Points to the right in film frame
        auto uu = Vec3::unit(Vec3::cross(vup, ww));
        // Points up in film frame
        auto vv = Vec3::cross(ww, uu);

        auto theta = vfov * PI / 180.0f;
        auto half_height = std::tan(theta * 0.5);
        auto half_width = aspect * half_height;

        lower_left = look_from - half_width * uu * focus_distance -
            half_height * vv * focus_distance - ww * focus_distance;
        hori = 2.0f * half_width * uu * focus_distance;
        vert = 2.0f * half_height * vv * focus_distance;
        origin = look_from;
        lens_radius = aperture * 0.5;
        u = uu;
        v = vv;
        _w = ww;
    }

    Ray Camera :: ray(const Float s, const Float t) const
    {
        Vec3 rd = Vec3::origin();
        if(lens_radius > 0.0f)
        {
            rd = lens_radius * Vec3::randInUnitDisk();
        }

        const auto offset = u * rd[0] + v * rd[1];
        return Ray{ origin + offset,
                    lower_left + s * hori + t * vert - origin - offset };
    }

} // namespace lorentz
