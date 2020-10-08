#include <utility>

#include "primitive_trait.h"

namespace lorentz
{
    bool BBox :: hit(const Ray& r, const Float tmin, const Float tmax) const
    {
        for(size_t i = 0; i < 3; i++)
        {
            const Float dir_inverse = 1.0f / r.dir[i];
            auto t0 = (lower[i] - r.origin[i]) * dir_inverse;
            auto t1 = (higher[i] - r.origin[i]) * dir_inverse;
            if(t0 > t1)
            {
                std::swap(t0, t1);
            }

            auto the_tmin = t0 > tmin ? t0 : tmin;
            auto the_tmax = t1 < tmax ? t1 : tmax;

            if(the_tmax <= the_tmin) { return false; }
        }
        return true;
    }

    BBox BBox :: union_box(const BBox& rhs) const
    {
        return BBox(Vec3(lower[0] < rhs.lower[0] ? lower[0] : rhs.lower[0],
                         lower[1] < rhs.lower[1] ? lower[1] : rhs.lower[1],
                         lower[2] < rhs.lower[2] ? lower[2] : rhs.lower[2]),
                    Vec3(higher[0] < rhs.higher[0] ? rhs.higher[0] : higher[0],
                         higher[1] < rhs.higher[1] ? rhs.higher[1] : higher[1],
                         higher[2] < rhs.higher[2] ? rhs.higher[2] : higher[2]));
    }

    std::ostream& operator<<(std::ostream& out, const BBox& box)
    {
        out << box.lower << " --> " << box.higher;
        return out;
    }

} // namespace lorentz
