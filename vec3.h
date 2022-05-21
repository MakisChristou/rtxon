#ifndef VEC3_H
#define VEC3_H

#include "math.h"
#include <iostream>

class vec3
{
    public:
        double x;
        double y;
        double z;


        vec3()
        {

        }

        vec3(int X, int Y, int Z)
        {
            x = (double) X;
            y = (double) Y;
            z = (double) Z;
        }

        vec3(double X, double Y, double Z)
        {
            x = X;
            y = Y;
            z = Z;
        }

        inline double magnitude()
        {
            return sqrt(x*x + y*y + z*z);
        }

        inline vec3 cross(vec3 v)
        {
           return vec3  (y * v.z - z * v.y,
                        z * v.x - x * v.z,
                        x * v.y - y * v.x);
        }

        inline vec3 normalize()
        {
            double m = magnitude();

            return vec3(x/m, y/m, z/m);
        }

        inline double angle(vec3 v)
        {
            return acos(( x * v.x + y * v.y + z * v.z ) / (magnitude() * v.magnitude() ));
        }

        inline double lengthSquared()
        {
            return (x * x + y * y + z * z);
        }


        inline double dot(vec3 v)
        {
            return (x * v.x + y * v.y + z * v.z);
        }

        vec3 operator-() const { return vec3(-x, -y, -z); }

        vec3& operator+=(const vec3 &v) {
            x += v.x;
            y += v.y;
            z += v.z;
            return *this;
        }

        vec3& operator*=(const double t) {
            x *= t;
            y *= t;
            z *= t;
            return *this;
        }

        vec3& operator/=(const double t) {
            return *this *= 1/t;
        }

        // Operator overloading for readability

        vec3 operator + (vec3& v)
        {
            return vec3(x + v.x, y + v.y, z + v.z);
        }

        vec3 operator - (vec3& v)
        {
            return vec3(x - v.x, y - v.y, z - v.z);
        }

        vec3 operator * (vec3& v)
        {
            return vec3(x * v.x, y * v.y, z * v.z);
        }

        vec3 operator * (double a)
        {
            return vec3(x * a, y * a, z * a);
        }

        vec3 operator / (vec3& v)
        {
            return vec3(x / v.x, y / v.y, z / v.z);
        }

        vec3 operator / (double a)
        {
            return vec3(x / a, y / a, z / a);
        }

};

// vec3 Utility Functions

inline std::ostream& operator<<(std::ostream &out, const vec3 &v) {
    return out << v.x << ' ' << v.y << ' ' << v.z;
}

inline vec3 operator+(const vec3 &u, const vec3 &v) {
    return vec3(u.x + v.x, u.y + v.y, u.z + v.z);
}

inline vec3 operator-(const vec3 &u, const vec3 &v) {
    return vec3(u.x - v.x, u.y - v.y, u.z - v.z);
}

inline vec3 operator*(const vec3 &u, const vec3 &v) {
    return vec3(u.x * v.x, u.y * v.y, u.z * v.z);
}

inline vec3 operator*(double t, const vec3 &v) {
    return vec3(t*v.x, t*v.y, t*v.z);
}

inline vec3 operator*(const vec3 &v, double t) {
    return t * v;
}

inline vec3 operator/(vec3 v, double t) {
    return (1/t) * v;
}

inline double dot(const vec3 &u, const vec3 &v) {
    return u.x * v.x
         + u.y * v.y
         + u.z * v.z;
}

inline vec3 cross(const vec3 &u, const vec3 &v) {
    return vec3(u.y * v.z - u.z * v.y,
                u.z * v.x - u.x * v.z,
                u.x * v.y - u.y * v.x);
}

// inline vec3 unit_vector(vec3 v) {
//     return v / v.magnitude();
// }

#endif