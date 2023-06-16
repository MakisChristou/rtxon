# rtxon
Yet another Rust raytracer implementation. The goal of this project is to familiarize myself with more intermidiate concepts in Rust such as traits, lifetimes, shared pointers and design patterns as well as learn a bit of Raytracing basics on the side. The source code is my implementation of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.


## Features
- Basic Materials (Lambertian, Metal, Dielectric, Emmisive)
- Shapes (Sphere, Rectangle)
- Positionable Camera
- Anti Aliasing
- Depth of Field
- Motion Blur
- Cute progress bar when rendering

## Future Features
- Proper BHV implementation
- Parallelism
- GUI progress indicator
- OBJ Import (required triangle intersection implementation)
- HDR background for global illumination

## Rendering default scene
```bash
cargo run --release
```

## Renders
### 1280x720, aa = 1024, depth = 100, f = 0.05, ~10min (M2 Macbook Pro)
<img src="images/render6.png" alt="Scene 1" width="650">

### 640x360, aa = 512, depth = 100, f = 0.05, ~44s (M2 Macbook Pro)
![Scene 1](images/render1.png)

### 640x360, aa = 512, depth = 100, f = 0.2, ~45s (M2 Macbook Pro)
![Scene 2](images/render2.png)

### 640x360, aa = 512, depth = 100, f = 1.0, ~65s (M2 Macbook Pro)
![Scene 3](images/render3.png)

### 640x360, aa = 128, depth = 100, f = 0.05, ~12s (M2 Macbook Pro)
![Scene 4](images/render4.png)

### 640x360, aa = 512, depth = 100, f = 0.1, ~7min (M2 Macbook Pro)
![Scene 5](images/render5.png)
