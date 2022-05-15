#ifndef SPHERE_H
#define SPHERE_H

#include "vec3.h"
#include "ray.h"

class Sphere
{
    public:
        vec3 origin;
        double radius;


        Sphere()
        {

        }

        Sphere(vec3 o, double r)
        {
            origin = o;
            radius = r;
        }

        bool intersection(Ray r)
        {


            return false;
        }

        

};


#endif