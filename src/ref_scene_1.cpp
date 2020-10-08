#include "primitive.h"
#include "ref_scene_1.h"
#include "random.h"

namespace lorentz::ref_scene_1
{
    namespace
    {
        PrimitiveList buildPrimitives(
            const std::vector<MaterialPtr>& materials)
        {
            PrimitiveList result;
            result.bounded.emplace_back(new Sphere(Vec3(0.0f, 0.0f, -1.0f), 0.5f));
            result.bounded.back()->material = materials[0];

            result.bounded.emplace_back(new Sphere(Vec3(1.0f, 0.0f, -1.0f), 0.5f));
            result.bounded.back()->material = materials[3];

            result.bounded.emplace_back(new Sphere(Vec3(-1.0f, 0.0f, -1.0f), 0.5f));
            result.bounded.back()->material = materials[1];

            for(int i = 0; i < 20; i++)
            {
                Float x = -5.0f + 0.5f * i;
                for(int j = 0; j < 20; j++)
                {
                    Float z = -5.0f + 0.5f * j;
                    auto sphere = std::make_shared<Sphere>(Vec3(x, -0.4f, z), 0.1f);
                    sphere -> material = materials[(i * 20 + j) % materials.size()];
                    result.bounded.emplace_back(sphere);
                }
            }

            result.unbounded.emplace_back(
                new InfinitePlane(Vec3(0.0f, -0.5f, 0.0f), Vec3(0.0f, 1.0f, 0.0f)));
            result.unbounded.back()->material = materials[2];
            return result;
        }

    } // namespace

    Scene buildScene(const uint32_t width, const uint32_t height)
    {
        auto camera_pos = Vec3(3.5f, 0.35f, 1.0f);
        auto camera_lookat = Vec3(0.0f, -0.4f, -1.0f);

        Scene scene;
        scene.width = width;
        scene.height = height;
        scene.camera = Camera(camera_pos, camera_lookat, Vec3(0.0f, 1.0f, 0.0f),
                              40.0f, Float(width) / Float(height), 0.06f,
                              (camera_lookat - camera_pos).norm() - 0.5f);

        scene.materials.emplace_back(new Metal(Vec3(0.5f, 0.5f, 0.5f), 0.0f));
        scene.materials.emplace_back(new Lambertian(Vec3(0.7, 0.7, 0.2)));
        scene.materials.emplace_back(new Lambertian(Vec3(0.5, 0.5, 0.5)));
        scene.materials.emplace_back(new Glass(1.5f));
        scene.materials.emplace_back(new Glass(1.7f));
        scene.materials.emplace_back(new Glass(1.7f));
        scene.materials.emplace_back(new Metal(Vec3(0.4, 0.5, 0.6), 0.1));

        for(int i = 0; i < 10; i++)
        {
            scene.materials.emplace_back(new LambertianRandomColor());
        }

        scene.primitives = buildPrimitives(scene.materials);
        scene.primitives.buildBvh();
        return scene;
    }


} // namespace lorentz::ref_scene_1
