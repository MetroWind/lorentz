#include <cmath>

#include "primitive.h"

namespace lorentz
{
    std::optional<Hit> Sphere :: intersect(
        const Ray& r, const Float t_min, const Float t_max) const
    {
        auto oc = r.origin - center;
        auto a = r.dir.normSquared();
        auto b = Vec3::dot(oc, r.dir);
        auto c = oc.normSquared() - radius * radius;

        auto discriminant = b * b - a * c;

        if(discriminant > 0.0f)
        {
            auto temp = (-b - std::sqrt(discriminant)) / a;
            if(!(temp < t_max && temp > t_min))
            {
                temp = (-b + std::sqrt(discriminant)) / a;
                if(!(temp < t_max && temp > t_min))
                {
                    return std::nullopt;
                }
            }

            auto p = r.at(temp);
            return Hit{ temp, p, (p - center) / radius, material};
        }
        return std::nullopt;
    }

    BBox Sphere :: bbox() const
    {
        return BBox(center - Vec3(radius, radius, radius),
                    center + Vec3(radius, radius, radius));
    }

    std::optional<Hit> InfinitePlane :: intersect(
        const Ray& r, const Float t_min, const Float t_max) const
    {
        auto denomi = Vec3::dot(normal, r.dir);
        if(denomi == 0.0f)
        {
            // Ray is parallel to plane.
            return std::nullopt;
        }
        auto t = Vec3::dot(origin - r.origin, normal) / denomi;
        if(t < t_max && t > t_min)
        {
            return Hit{ t, r.at(t), normal, material };
        }
        else
        {
            return std::nullopt;
        }
    }

}
