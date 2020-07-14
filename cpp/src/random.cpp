#include "random.h"

namespace lorentz
{
    Float random(Float min, Float max)
    {
        std::uniform_real_distribution<Float> Distro(min, max);
        return Distro(LocalRandGen);
    }

    size_t random(size_t min, size_t max)
    {
        std::uniform_int_distribution<size_t> Distro(min, max);
        return Distro(LocalRandGen);
    }

} // namespace lorentz
