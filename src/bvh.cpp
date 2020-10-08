#include <algorithm>
#include <iostream>

#include "bvh.h"
#include "random.h"

namespace lorentz
{
    BvhNodePtr BvhNode :: build(std::span<BoundedPrimitivePtr> prims)
    {
        const size_t len = prims.size();
        const size_t ax = static_cast<size_t>(random(0.0f, 3.0f));
        BvhNodePtr node = std::make_unique<BvhNode>();

        if(len == 1)
        {
            node -> obj = prims[0];
            node -> bounding_box = prims[0] -> bbox();
        }
        else
        {
            std::sort(
                std::begin(prims), std::end(prims),
                [ax](const BoundedPrimitivePtr& a, const BoundedPrimitivePtr& b)
                {
                    return a -> bbox().lower[ax] < b -> bbox().lower[ax];
                });

            node -> left = build(prims.subspan(0, len / 2));
            node -> right = build(prims.subspan(len / 2, len - len / 2));

            // std::cout << "Bounding box for node is \n\t" << node->left->bbox()
            //           << "+\n\t" << node->right->bbox() << " ==> \n\t"
            //           << node->left->bbox().union_box(node->right->bbox()) << std::endl;

            node -> bounding_box = node -> left -> bbox().union_box(
                node -> right -> bbox());
        }
        return node;
    }

    std::optional<Hit> BvhNode :: intersect(
            const Ray& r, const Float t_min, const Float t_max) const
    {
        if(!bounding_box.hit(r, t_min, t_max))
        {
            return std::nullopt;
        }
        if(isLeaf())
        {
            return obj -> intersect(r, t_min, t_max);
        }
        else
        {
            auto left_hit = left -> intersect(r, t_min, t_max);
            auto right_hit = right -> intersect(r, t_min, t_max);

            if(left_hit.has_value() && right_hit.has_value())
            {
                if(left_hit -> t < right_hit -> t)
                {
                    return left_hit;
                }
                else
                {
                    return right_hit;
                }
            }
            else if(left_hit.has_value())
            {
                return left_hit;
            }
            else if(right_hit.has_value())
            {
                return right_hit;
            }
            else
            {
                return std::nullopt;
            }
        }
    }


} // namespace lorentz
