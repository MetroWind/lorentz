// -*- mode: c++; -*-
#ifndef LORENTZ_RANDOM_H
#define LORENTZ_RANDOM_H

#include <random>

#include "config.h"

namespace lorentz
{
    static thread_local std::mt19937 LocalRandGen;

    Float random(Float min, Float max);
    inline Float random() { return random(0.0f, 1.0f); }

} // namespace lorentz
#endif
