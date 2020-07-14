// -*- mode: c++; -*-
#ifndef LORENTZ_VEC3_H
#define LORENTZ_VEC3_H

#include <array>
#include <iostream>

#include "config.h"
#include "random.h"

namespace lorentz
{
    class Vec3
    {
    public:
        Vec3() = default;
        Vec3(Float x, Float y, Float z)
                : data({x, y, z}) {}

        static inline Vec3 unit(const Vec3& v);
        static inline Vec3 origin();
        static inline Vec3 randInUnitSphere();
        static inline Vec3 randInUnitDisk();

        static inline Float dot(const Vec3& lhs, const Vec3& rhs);
        static inline Vec3 cross(const Vec3& lhs, const Vec3& rhs);

        inline Float operator[](size_t i) const;
        inline Float& operator[](size_t i);

        inline Vec3 operator+(const Vec3& rhs) const;
        inline Vec3& operator+=(const Vec3& rhs);
        inline Vec3 operator-(const Vec3& rhs) const;
        inline Vec3& operator-=(const Vec3& rhs);
        inline Vec3 operator*(const Vec3& rhs) const;
        inline Vec3 operator*(const Float rhs) const;
        inline Vec3& operator*=(const Vec3& rhs);
        inline Vec3& operator*=(const Float rhs);
        inline Vec3 operator/(const Vec3& rhs) const;
        inline Vec3 operator/(const Float rhs) const;
        inline Vec3& operator/=(const Vec3& rhs);
        inline Vec3& operator/=(const Float rhs);

        inline Vec3 operator-() const;

        inline Float normSquared() const;
        inline Float norm() const;

        friend inline Vec3 operator*(const Float scale, const Vec3& v);
    private:
        std::array<Float, 3> data;

    };

    std::ostream& operator<<(std::ostream& out, const Vec3& v);

    using Color = Vec3;

    inline Vec3 Vec3 :: unit(const Vec3& v)
    {
        return v / v.norm();
    }

    inline Vec3 Vec3 :: origin()
    {
        return Vec3(0.0f, 0.0f, 0.0f);
    }

    inline Vec3 Vec3 :: randInUnitSphere()
    {
        Vec3 p;
        while(true)
        {
            p = 2.0f * Vec3(random(), random(), random()) - Vec3(1.0f, 1.0f, 1.0f);
            if(p.normSquared() < 1.0f)
            {
                return p;
            }
        }
    }

    inline Vec3 Vec3 :: randInUnitDisk()
    {
        while(true)
        {
            Vec3 p = 2.0f * Vec3(random(), random(), 0.0f) - Vec3(1.0f, 1.0f, 0.0f);
            if(p.normSquared() < 1.0f)
            {
                return p;
            }
        }
    }

    inline Float Vec3 :: operator[](size_t i) const
    {
        return data[i];
    }

    inline Float& Vec3 :: operator[](size_t i)
    {
        return data[i];
    }

    inline Vec3 Vec3 :: operator+(const Vec3& rhs) const
    {
        return Vec3(data[0] + rhs.data[0],
                    data[1] + rhs.data[1],
                    data[2] + rhs.data[2]);
    }

    inline Vec3& Vec3 :: operator+=(const Vec3& rhs)
    {
        data[0] += rhs.data[0];
        data[1] += rhs.data[1];
        data[2] += rhs.data[2];
        return *this;
    }

    inline Vec3 Vec3 :: operator-(const Vec3& rhs) const
    {
        return Vec3(data[0] - rhs.data[0],
                    data[1] - rhs.data[1],
                    data[2] - rhs.data[2]);
    }

    inline Vec3& Vec3 :: operator-=(const Vec3& rhs)
    {
        data[0] -= rhs.data[0];
        data[1] -= rhs.data[1];
        data[2] -= rhs.data[2];
        return *this;
    }

    inline Vec3 Vec3 :: operator*(const Vec3& rhs) const
    {
        return Vec3(data[0] * rhs.data[0],
                    data[1] * rhs.data[1],
                    data[2] * rhs.data[2]);
    }

    inline Vec3 Vec3 :: operator*(const Float rhs) const
    {
        return Vec3(data[0] * rhs, data[1] * rhs, data[2] * rhs);
    }

    inline Vec3& Vec3 :: operator*=(const Vec3& rhs)
    {
        data[0] *= rhs.data[0];
        data[1] *= rhs.data[1];
        data[2] *= rhs.data[2];
        return *this;
    }

    inline Vec3& Vec3 :: operator*=(const Float rhs)
    {
        data[0] *= rhs;
        data[1] *= rhs;
        data[2] *= rhs;
        return *this;
    }

    inline Vec3 Vec3 :: operator/(const Vec3& rhs) const
    {
        return Vec3(data[0] / rhs.data[0],
                    data[1] / rhs.data[1],
                    data[2] / rhs.data[2]);
    }

    inline Vec3 Vec3 :: operator/(const Float rhs) const
    {
        const Float scale = 1.0f / rhs;
        return Vec3(data[0] * scale, data[1] * scale, data[2] * scale);
    }

    inline Vec3& Vec3 :: operator/=(const Vec3& rhs)
    {
        data[0] /= rhs.data[0];
        data[1] /= rhs.data[1];
        data[2] /= rhs.data[2];
        return *this;
    }

    inline Vec3& Vec3 :: operator/=(const Float rhs)
    {
        const Float scale = 1.0f / rhs;
        data[0] *= scale;
        data[1] *= scale;
        data[2] *= scale;
        return *this;
    }

    inline Vec3 Vec3 :: operator-() const
    {
        return Vec3(-data[0], -data[1], -data[2]);
    }

    inline Vec3 operator*(const Float scale, const Vec3& v)
    {
        return Vec3(scale * v.data[0], scale * v.data[1], scale * v.data[2]);
    }


    inline Float Vec3 :: normSquared() const
    {
        return data[0] * data[0] + data[1] * data[1] + data[2] * data[2];
    }

    inline Float Vec3 :: norm() const
    {
        return std::sqrt(normSquared());
    }

    inline Float Vec3 :: dot(const Vec3& lhs, const Vec3& rhs)
    {
        return lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2];
    }

    inline Vec3 Vec3 :: cross(const Vec3& lhs, const Vec3& rhs)
    {
        return Vec3(lhs[1] * rhs[2] - lhs[2] * rhs[1],
                    lhs[2] * rhs[0] - lhs[0] * rhs[2],
                    lhs[0] * rhs[1] - lhs[1] * rhs[0]);
    }


} // namespace lorentz

#endif
