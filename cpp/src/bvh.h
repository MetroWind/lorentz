// -*- mode: c++; -*-
#ifndef LORENTZ_BVH_H
#define LORENTZ_BVH_H

#include <memory>
#include <vector>
#include <span>

#include "config.h"
#include "ray.h"
#include "primitive_trait.h"
#include "random.h"

namespace lorentz
{
    class BvhNode;
    using BvhNodePtr = std::unique_ptr<BvhNode>;

    class BvhNode: public BoundedPrimitive
    {
    public:
        bool isLeaf() const { return obj != nullptr; }

        // Look ma, I’m using C++20!
        static BvhNodePtr build(std::span<BoundedPrimitivePtr> prims);

        virtual std::optional<Hit> intersect(
            const Ray& r, const Float t_min, const Float t_max) const override;
        virtual BBox bbox() const override { return bounding_box; }

    private:
        BvhNodePtr left = nullptr;
        BvhNodePtr right = nullptr;
        BoundedPrimitivePtr obj = nullptr;
        BBox bounding_box;
    };


} // namespace lorentz

#endif
