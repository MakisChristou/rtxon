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

#endif