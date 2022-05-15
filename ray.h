#ifndef RAY_H
#define RAY_H

#include "vec3.h"

class Ray
{
    public:
        vec3 origin;
        vec3 direction;


        Ray()
        {

        }

        Ray(vec3 o, vec3 d)
        {
            origin = o;
            direction = d;
        }

        
};


#endif