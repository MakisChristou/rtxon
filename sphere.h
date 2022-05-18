#ifndef SPHERE_H
#define SPHERE_H

#include "vec3.h"
#include "ray.h"

class Sphere
{
    public:
        vec3 center;
        double radius;


        Sphere()
        {

        }

        Sphere(vec3 o, double r)
        {
            center = o;
            radius = r;
        }

        vec3 normal(vec3 P)
        {
            return (P - center);
        }

        vec3 getCenter()
        {
            return center;
        }

        double getRadius()
        {
            return radius;
        }



        

};


#endif