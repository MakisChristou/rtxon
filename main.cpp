#include <iostream>


#include "vec3.h"
#include "ray.h"


#include <SDL2/SDL.h>

int main(int argc, char** argv)
{

    std::cout << "Hello World" << std::endl;

    vec3 testVector1(1,2,3);
    vec3 testVector2(4,5,6);

    vec3 testVector3 = testVector1.cross(testVector2);

    std::cout << testVector3.x << " " << testVector3.y << " " << testVector3.z << std::endl;

    int image_width = 640;
    int image_height = 480;

    for(int i = 0; i < image_width; i++)
    {
        for(int j = 0; j < image_height; j++)
        {
            // Raytrace here

        }
    }


    return 0;
}