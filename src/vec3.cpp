#include "vec3.h"
#include <cmath>

namespace lorentz
{
    std::ostream& operator<<(std::ostream& out, const Vec3& v)
    {
        out << "(" << v[0] << ", " << v[1] << ", " << v[2] << ")";
        return out;
    }

} // namespace lorentz
