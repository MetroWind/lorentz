#include "random.h"

namespace lorentz
{
    Float random(Float min, Float max)
    {
        std::uniform_int_distribution<Float> Distro(min, max);
        return Distro(LocalRandGen);
    }

} // namespace lorentz
