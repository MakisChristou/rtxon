#ifndef CAMERA_H
#define CAMERA_H

#include "vec3.h"
#include "ray.h"

class Camera
{
    public:
        vec3 origin;
        Ray currentRay;
        vec3 P[4];

        Camera()
        {

        }

        Camera(vec3 o, vec3 P0, vec3 P1, vec3 P2, vec3 P3)
        {
            origin = o;
            P[0] = P0;
            P[1] = P1;
            P[2] = P2;
            P[3] = P3;
        }

        // Convert pixel coords to complex coords
        inline long double map(long double input, long double output_start, long double output_end, long double input_start, long double input_end)
        {
            return (output_start + ((output_end - output_start) / (input_end - input_start)) * (input - input_start));
        }

        Ray getRay(int i, int j, int image_width, int image_height)
        {
            Ray currentRay;

            currentRay.origin = origin; // Set to camera origin

            // Map X coordinate
            double u = map(i, P[0].x, P[1].x, 0, image_width);

            // Map Y coordinate
            double v = map(j, P[0].z, P[2].z, 0, image_height);

            
            vec3 screenPoint(u, P[0].y, v);

            currentRay.direction = screenPoint;

            return currentRay;


        }


};


#endif