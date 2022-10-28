#include <iostream>
#include <SDL2/SDL.h>
#include <math.h>
#include <stdio.h>
#include <vector>
#include <random>

#include "vec3.h"
#include "ray.h"
#include "sphere.h"
#include "camera.h"

inline double random_double()
{
    // Returns a random real in [0,1).
    return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max)
{
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}

double sphereInter(Ray& ray, Sphere& sphere)
{

    // a = ||d||^2
    // b = 2(p · d)
    // c = ||p||^2 - r^2

    vec3 oc  =  ray.origin - sphere.center;
    double a = ray.direction.dot(ray.direction);
    double b = 2.0 * oc.dot(ray.direction);
    double c = oc.dot(oc) - sphere.radius*sphere.radius;

    double Delta = b*b - 4 * a * c;
    double t1 = 0.0;

    if(Delta < 0.0f)
    {
        return -1.0;;
    }
    else
    {
        return (-b - sqrt(Delta) ) / (2.0*a);;
    }
}


typedef struct 
{
    int R;
    int G;
    int B;
}Color;

vec3 getRandomPoint(vec3 P, vec3 N)
{
    // Calculate unit circle outside of sphere
    Sphere unitSphere(P + N, 1.0);

    vec3 S;

    while(true)
    {
        S = vec3(random_double(0,1.0), random_double(0,1.0), random_double(0,1.0));

        if(S.lengthSquared() >= 1.0)
        {
            return S;
        }
    }

}

int getClosestSphere(std::vector<Sphere>& Spheres, Ray& r)
{
    float tmin = MAXFLOAT;
    int i = 0;
    int minIndex = -1;

    for(auto sphere : Spheres)
    {
        double t = sphereInter(r, sphere);

        if(t < tmin && t > 0)
        {
            tmin = t;
            minIndex = i;
        }
        i++;
    }
    return minIndex;
}


vec3 colorPixel(std::vector<Sphere>& Spheres, Ray& r, int depth)
{
    if (depth <= 0)
        return vec3{0,0,0};


    int index = getClosestSphere(Spheres, r);

    // Hit any sphere
    if(index != -1)
    {
        Sphere hitSphere = Spheres[index];
        
        double t = sphereInter(r,hitSphere);

        // Bounce new ray
        
        vec3 C = hitSphere.center;

        vec3 P = r.getOrigin() + r.getDirection() * t; 

        vec3 N = hitSphere.normal(P);

        vec3 S = getRandomPoint(P, N);

        // New ray will have origin = P and direction = S

        Ray rr(P,S);

        return 0.5 * colorPixel(Spheres, rr, depth -1);


    }
    // Paint background
    else
    {
        vec3 unit_direction = r.getDirection().normalize();
        auto t = 0.5*(unit_direction.y + 1.0);
        return (1.0 - t) * vec3(255, 255, 255) + t * vec3(125, 200, 255);
    }

    
}

// Prints PPM in std
void writePPM(Color* pixelColors, int image_width, int image_height)
{
    printf("P3\n");
    printf("%d %d",image_width,image_height);
    printf("\n255\n");

    for(int j = image_height-1; j >=0; --j)
    {
        for(int i = 0; i < image_width; ++i)
        {
            Color c = pixelColors[i*image_height + j];
            printf("%d %d %d\n", c.R, c.G, c.B);
        }
    }
}

int main(int argc, char** argv)
{   
    int image_width  = 800;
    int image_height = 400;
    int s_max = 128;
    int depth = 100;

    Camera cam(vec3{0.0, 0.0 ,-0.4}, vec3{-2,1,-1}, vec3{2,1,-1}, vec3{-2,-1,-1}, vec3{2,-1,-1});

    Sphere s1(vec3{0.0, 0.0, -1.0}, 0.2);
    Sphere s2(vec3{0.0, -100.2, -1.0}, 100);
    Sphere s3(vec3{0.5, 0.0, -1.0}, 0.2);
    Sphere s4(vec3{-0.5, 0.0, -1.0}, 0.2);

    std::vector<Sphere> Spheres{s1, s2, s3, s4};

    Color white{255, 255, 255};
    Color red{255, 0, 0};
    Color blue{0, 0, 255};
    Color green{0, 255, 0};
    Color black{0, 0, 0};

    // Allocate memory for pixel colors
    Color* pixelColors = new Color[image_width * image_height];
    

    for(int i = 0; i < image_width; i++)
    {
        for(int j = 0; j < image_height; j++)
        {

            vec3 pixelVec3Sum{0, 0, 0};

            // Anti Aliasing
            for(double k = 0.0; k < 1.0; k+=1.0/s_max)
            {
                // Get current ray
                double ii  = i+k;
                double jj = j+k;
                Ray r = cam.getRay(ii, jj, image_width, image_height);

                pixelColors[i*image_height + j] = black;

                pixelVec3Sum = pixelVec3Sum + colorPixel(Spheres, r, depth);
            }

            pixelVec3Sum.x /= s_max;
            pixelVec3Sum.y /= s_max;
            pixelVec3Sum.z /= s_max;

            pixelColors[i*image_height + j] = Color{pixelVec3Sum.x, pixelVec3Sum.y, pixelVec3Sum.z};
        }
    }


    writePPM(pixelColors, image_width, image_height);


    return 0;
}