// -*- mode: c++; -*-
#ifndef LORENTZ_MATERIAL_H
#define LORENTZ_MATERIAL_H

#include <optional>

#include "config.h"
#include "vec3.h"
#include "random.h"
#include "ray.h"

namespace lorentz
{
    Vec3 reflect(const Vec3& in, const Vec3& normal);
    std::optional<Vec3> refract(const Vec3& in, const Vec3& normal,
                                const Float ni_over_nt);
    Float schlick(const Float cos, const Float ref_idx);

    class Material
    {
    public:
        virtual ~Material() {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const = 0;
    };

    class Lambertian : public Material
    {
    public:
        virtual ~Lambertian() override {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const override;

        Vec3 albedo;
    };

    class LambertianRandomColor : public Lambertian
    {
    public:
        LambertianRandomColor();
    private:
        Color randomColor() const;
    };

    class Metal : public Material
    {
    public:
        virtual ~Metal() override {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const override;

        Vec3 albedo;
        Float roughness = 0.0f;
    };

    class Glass : public Material
    {
    public:
        virtual ~Glass() override {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const override;

        Float ref_index = 1.0f;
    };

} // namespace lorentz

#endif
