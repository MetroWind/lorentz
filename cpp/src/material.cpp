#include "material.h"
#include "utils.h"

namespace lorentz
{
    Vec3 reflect(const Vec3& in, const Vec3& normal)
    {
        return in - 2.0f * Vec3::dot(in, normal) * normal;
    }

    std::optional<Vec3> refract(const Vec3& in, const Vec3& normal,
                                const Float ni_over_nt)
    {
        auto uv = Vec3::unit(in);
        auto dt = Vec3::dot(uv, normal);
        auto discriminant = 1.0f - ni_over_nt * ni_over_nt * (1.0f - dt * dt);
        if(discriminant > 0.0f)
        {
            return ni_over_nt * (uv - normal*dt) - normal * std::sqrt(discriminant);
        }
        else
        {
            return std::nullopt;
        }
    }

    Float schlick(const Float cos, const Float ref_idx)
    {
        auto r0 = (1.0f - ref_idx) / (1.0f + ref_idx);
        auto r02 = r0 * r0;
        return r02 + (1.0f - r02) * std::pow(1.0f - cos, 5);
    }


    std::optional<std::pair<Ray, Vec3>>
    Lambertian :: scatter(const Ray& in, const Hit& hit) const
    {
        UNUSED(in);
        auto target = hit.p + hit.normal + Vec3::randInUnitSphere();
        return std::make_pair(Ray{ hit.p, target - hit.p },
                              albedo);
    }

    LambertianRandomColor :: LambertianRandomColor()
    {
        albedo = randomColor();
    }

    Color LambertianRandomColor :: randomColor() const
    {
        return Color(random(0.1, 0.7), random(0.1, 0.7), random(0.1, 0.7));
    }

    std::optional<std::pair<Ray, Vec3>>
    Metal :: scatter(const Ray& in, const Hit& hit) const
    {
        auto reflected = reflect(Vec3::unit(in.dir), hit.normal);
        auto scattered = Ray{
            hit.p,
            roughness * Vec3::randInUnitSphere() + reflected,
        };

        if(Vec3::dot(scattered.dir, hit.normal) > 0.0f)
        {
            return std::make_pair(scattered, albedo);
        }
        else
        {
            return std::nullopt;
        }
    }

    std::optional<std::pair<Ray, Vec3>>
    Glass :: scatter(const Ray& in, const Hit& hit) const
    {
        Vec3 ref_normal = Vec3::origin();
        Float ni_over_nt = 0.0f;
        Vec3 attenuation = Vec3(1.0f, 1.0f, 1.0f);
        Float cos = 0.0f;

        if(Vec3::dot(in.dir, hit.normal) > 0.0f)
        {
            ref_normal = -(hit.normal);
            ni_over_nt = ref_index;
            cos = ref_index * Vec3::dot(in.dir, hit.normal) / in.dir.norm();
        }
        else
        {
            ref_normal = hit.normal;
            ni_over_nt = 1.0f / ref_index;
            cos = -Vec3::dot(in.dir, hit.normal) / in.dir.norm();
        }

        auto reflect_prob = schlick(cos, ref_index);
        auto refracted = refract(in.dir, ref_normal, ni_over_nt);
        if(refracted.has_value())
        {
            if(random() < reflect_prob)
            {
                auto reflected = reflect(in.dir, hit.normal);
                return std::make_pair(Ray{hit.p, reflected}, attenuation);
            }
            else
            {
                return std::make_pair(Ray{hit.p, *refracted}, attenuation);
            }
        }
        else
        {
            auto reflected = reflect(in.dir, hit.normal);
            return std::make_pair(Ray{hit.p, reflected}, attenuation);
        }
    }
} // namespace lorentz
