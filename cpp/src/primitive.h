// -*- mode: c++; -*-
#ifndef LORENTZ_PRIMITIVE_H
#define LORENTZ_PRIMITIVE_H

#include "vec3.h"
#include "primitive_trait.h"
#include "material.h"

namespace lorentz
{
    class Sphere: public BoundedPrimitive
    {
    public:
        Sphere(const Vec3& c, const Float r);
        virtual ~Sphere() override {}

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;
        virtual BBox bbox() const override;

    private:
        Vec3 center;
        Float radius;
        Material* material;
    };

    class InfinitePlane: public Primitive
    {
    public:
        InfinitePlane(const Vec3& o, const Vec3 n);
        virtual ~InfinitePlane() override {}

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;

    private:
        Vec3 origin;
        Vec3 normal;
        Material* material;
    };

} // namespace lorentz

#endif
