# Keikan (警官)
*An elegant (imo) rendering engine written in Rust.*

Loosely based on [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html), Keikan is a photorealistic rendering engine that supports path-tracing, ray-marching, photo based rendering (through a principled implementation), and more.

I've tried my hand a few toy path-tracers and ray-marchers in the past, and I wanted to write something more than just a toy. Having just stumbled upon Rust, I decided to try my hand at it by implementing a rendering engine from scratch.

## Design Goals
Keikan is able to render a large variety of objects, including fractals to near-infinite precision, due to its support of both path-tracing and ray-marching.

Ray-marching is rendering through the use of a [signed distance field](https://iquilezles.org/www/articles/distfunctions/distfunctions.htm).
I personally got interested in computer graphics through writing ray-marching renderer, so I decided that if I wrote another rendering engine, I'd have include ray-marching as a technique.

## Results
Here's the most recent render, a ray-marched fractal known as a 'Mandelbulb'. As of right know, composing scenes is a bit janky, and the rendering code a bit unoptimized.

![](https://raw.githubusercontent.com/Tloru/keikan/master/renders/render%2012.png)
> Rendered in 222 seconds on a MacBook Air.

## Why (the Name) Keikan?
It's Japanese for policeman.
