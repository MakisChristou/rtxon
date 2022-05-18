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

inline double random_double() {
    // Returns a random real in [0,1).
    return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}

double sphereInter(Ray* ray, Sphere* sphere)
{

    // a = ||d||^2
    // b = 2(p · d)
    // c = ||p||^2 - r^2

    vec3 oc  =  ray->origin - sphere->center;
    double a = ray->direction.dot(ray->direction);
    double b = 2.0 * oc.dot(ray->direction);
    double c = oc.dot(oc) - sphere->radius*sphere->radius;

    double Delta = b*b - 4 * a * c;
    double t1,t2 = 0.0;

    if(Delta < 0.0f)
    {
        return -1;
    }
    else
    {
        //std::cerr << "t1: " << t1 << " ";
        return (-b - sqrt(Delta)) / (2*a);
    }
}


bool planeInter(Ray* ray, Sphere* Plane, vec3* point)
{
    return false;
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



vec3 colorPixel(Ray* r)
{


    
}


int getClosestSphere(std::vector<Sphere> *Spheres, Ray* r)
{
    float tmin = MAXFLOAT;
    int i = 0;
    int minIndex = -1;

    for(auto sphere : *Spheres)
    {

        double t = sphereInter(r,&sphere);

        if(t < tmin)
        {
            tmin = t;
            minIndex = i;
        }
        i++;
    }

    return minIndex;
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
    int image_width  = 400;
    int image_height = 400;

    Camera cam(vec3(0,-1,0), vec3(1,1,1), vec3(-1,1,1), vec3(1,1,-1), vec3(-1,1,-1));
    
    
    Sphere s1(vec3(0,4,0), 1.0);


    Sphere s2(vec3(0.0, 4.0, -100.0), 100);

    Sphere s3(vec3(2.5, 4.0, 0.0), 1.0);

    Sphere s4(vec3(-2.5, 4.0, 0.0), 1.0);

    Sphere s5(vec3(0.0, -2.5, 0.0), 1.0);


    std::vector<Sphere> Spheres;

    Spheres.push_back(s1);
    Spheres.push_back(s2);
    // Spheres.push_back(s3);
    // Spheres.push_back(s4);
    // Spheres.push_back(s5);

    // std::cerr << Spheres.size() << std::endl;
    



    Color white;
    white.R = 255;
    white.G = 255;
    white.B = 255;

    Color red;
    red.R = 255;
    red.G = 0;
    red.B = 0;

    Color blue;
    blue.R = 0;
    blue.G = 0;
    blue.B = 255;

    Color green;
    green.R = 0;
    green.G = 255;
    green.B = 0;  


    Color black;
    black.R = 0;
    black.G = 0;
    black.B = 0; 


    size_t pixelBytes = image_width*image_height*sizeof(Color);


    // Allocate memory for pixel colors
    Color *pixelColors = (Color*)malloc(pixelBytes); // pixelColours

    vec3 points[2];

    

    for(int i = 0; i < image_width; i++)
    {
        for(int j = 0; j < image_height; j++)
        {
            // Get current ray
            Ray r = cam.getRay(i,j,image_width, image_height);
            
            // std::cerr  << r.getOrigin().x  << " "<< r.getOrigin().y  << " " << r.getOrigin().z;
            // std::cerr << " -> " << r.getDirection().x  << " "<< r.getDirection().y  << " " << r.getDirection().z << " ";


            pixelColors[i*image_height + j] = black;

            bool flag = false;
            
            int index = getClosestSphere(&Spheres, &r);

            // double t = sphereInter(&r, &Spheres[index]);

            // vec3 temp = r.getDirection() * t;

            // vec3 P = r.getOrigin() + temp; // Point of intersection

            // temp = s2.getCenter();

            // vec3 N = P - temp; // Normal at intersection point

            // vec3 S = getRandomPoint(P, N);


            if(index > 0)
            {
                pixelColors[i*image_height + j] = white;
                flag = true;
            }


            // std::cerr << " flag: " << flag << std::endl;
           
        }
    }


    writePPM(pixelColors, image_width, image_height);


    return 0;
}