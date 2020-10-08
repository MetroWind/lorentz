// -*- mode: c++; -*-
#ifndef LORENTZ_PRIMITIVE_TRAIT_H
#define LORENTZ_PRIMITIVE_TRAIT_H

#include <optional>
#include <memory>
#include <iostream>

#include "config.h"
#include "ray.h"
#include "vec3.h"
#include "material.h"

namespace lorentz
{
    struct BBox
    {
        BBox() = default;
        BBox(const Vec3& l, const Vec3& h) : lower(l), higher(h) {}
        Vec3 lower;
        Vec3 higher;

        bool hit(const Ray& r, const Float tmin, const Float tmax) const;
        BBox union_box(const BBox& rhs) const;
    };

    std::ostream& operator<<(std::ostream& out, const BBox& box);

    class Primitive
    {
    public:
        virtual ~Primitive() {}
        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const = 0;
        MaterialPtr material = nullptr;
    };

    class BoundedPrimitive : public Primitive
    {
    public:
        virtual ~BoundedPrimitive() override {}
        virtual BBox bbox() const = 0;
    };

    using PrimitivePtr = std::shared_ptr<Primitive>;
    using BoundedPrimitivePtr = std::shared_ptr<BoundedPrimitive>;

}

#endif
