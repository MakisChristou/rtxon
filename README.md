# RTX On
Yet another Rust raytracer implementation. The goal of this project is to familiarize myself with more intermidiate concepts in Rust such as traits, lifetimes and design patterns. The source code is my implementation of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

## Rendering default scene
```bash
cargo run --release > file.ppm
```

## Renders

### Resolution: 1600x800, s_max = 64, depth = 100 
![Sample 1](images/render1.png)

### Resolution: 1600x800, s_max = 128, depth = 100 
![Sample 1](images/render2.png)
