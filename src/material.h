// -*- mode: c++; -*-
#ifndef LORENTZ_MATERIAL_H
#define LORENTZ_MATERIAL_H

#include <optional>
#include <memory>

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

    using MaterialPtr = std::shared_ptr<Material>;

    class Lambertian : public Material
    {
    public:
        Lambertian() = default;
        Lambertian(const Vec3& a) : albedo(a) {}
        Lambertian(const Lambertian&) = default;
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
        Metal(const Vec3& a, const Float r): albedo(a), roughness(r) {}
        virtual ~Metal() override {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const override;

        Vec3 albedo;
        Float roughness = 0.0f;
    };

    class Glass : public Material
    {
    public:
        Glass(const Float r): ref_index(r) {}
        virtual ~Glass() override {}

        virtual std::optional<std::pair<Ray, Vec3>>
        scatter(const Ray& in, const Hit& hit) const override;

        Float ref_index = 1.0f;
    };

} // namespace lorentz

#endif
