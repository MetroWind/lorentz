// -*- mode: c++; -*-
#ifndef LORENTZ_PRIMITIVE_H
#define LORENTZ_PRIMITIVE_H

#include "vec3.h"
#include "primitive_trait.h"
#include "material.h"
#include "bvh.h"

namespace lorentz
{
    class Sphere: public BoundedPrimitive
    {
    public:
        Sphere(const Sphere&) = default;
        Sphere(const Vec3& c, const Float r): center(c), radius(r) {}
        virtual ~Sphere() override {}

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;
        virtual BBox bbox() const override;

    private:
        Vec3 center;
        Float radius;
    };

    class InfinitePlane: public Primitive
    {
    public:
        InfinitePlane(const Vec3& o, const Vec3& n): origin(o), normal(n) {}
        virtual ~InfinitePlane() override {}

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;

    private:
        Vec3 origin;
        Vec3 normal;
    };

    class PrimitiveList: public Primitive
    {
    public:
        PrimitiveList() = default;
        PrimitiveList(PrimitiveList&& rhs) = default;
        PrimitiveList& operator=(PrimitiveList&&) = default;
        virtual ~PrimitiveList() override {}

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;

        void buildBvh() { bvh_tree = BvhNode::build(bounded); }

        std::vector<BoundedPrimitivePtr> bounded;
        std::vector<PrimitivePtr> unbounded;
        bool use_bvh = false;

    private:
        BvhNodePtr bvh_tree;
    };


} // namespace lorentz

#endif
