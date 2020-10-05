# Keikan (警官)
*An elegant (imo) rendering engine written in Rust.*

**Keikan** is a multithreaded photorealistic rendering engine that supports
path-tracing, ray-marching, Photo Based Rendering
(through a principled implementation), and more.

I've tried my hand a few toy path-tracers and ray-marchers in the past,
and I wanted to write something more than just a toy.
Having just stumbled upon Rust, I decided to try my hand at it by
implementing a rendering engine from scratch.

This project was originally inspired by
[*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html),
but after I knocked out the core engine, I've pulled from a number of different
sources and my own experience to develop it further.

## Results
Here's a recent render of a ray-marched *Mandelbulb* (a type of fractal):

![A Mandelbulb fractal](https://raw.githubusercontent.com/Tloru/keikan/master/renders/render%2012.png)
> Rendered in 222 seconds on a MacBook Air.

There are some images bundles in this repository under the `keikan/renders/`
folder, ordered chronologically. Go check them out!

## Design Goals
Keikan is able to render a large variety of objects, including fractals to
near-infinite precision, due to its support of both path-tracing
and ray-marching.

Ray-marching is rendering through the use of a
[signed distance field](https://iquilezles.org/www/articles/distfunctions/distfunctions.htm).
I personally got interested in computer graphics through writing ray-marching
renderer, so I decided that if I wrote another rendering engine,
I'd have include ray-marching as a technique.

## Getting Started
To render a small demo, clone this repository and run Keikan:

```bash
git clone https://github.com/slightknack/keikan.git
cd keikan
cargo run --release -- ~/Desktop/demo.png  # where to save the image
```

You should see some output right away. Keikan will spawn as many threads as
detected CPU cores, so it should be ~pretty~ relatively fast
(for non-GPU-based rendering code, haha).

I'd like to use GPU features, as I've made some ray marchers in GLSL,
but it seems like Rust doesn't have standardized GPU support.
If you know how to run Rust on the GPU, open an issue so we can discuss it
further!

## Why (the Name) Keikan?
It's Japanese for policeman.
