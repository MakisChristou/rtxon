#ifndef VEC3_H
#define VEC3_H

#include "math.h"

class vec3
{
    public:
        int x;
        int y;
        int z;


        vec3()
        {

        }

        vec3(int X, int Y, int Z)
        {
            x = X;
            y = Y;
            z = Z;
        }

        double magnitude()
        {
            return sqrt(x*x + y*y + z*z);
        }

        vec3 cross(vec3 v)
        {
           return vec3  (y * v.z - z * v.y,
                        z * v.x - x * v.z,
                        x * v.y - y * v.x);
        }


        double dot(vec3 v)
        {
            return (x * v.x + y * v.y + z * v.z);
        }


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