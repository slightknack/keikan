# Keikan (警官)
*An elegant (imo) rendering engine written in Rust.*

Loosely based on [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html), Keikan is a photorealistic rendering engine that supports path-tracing, ray-marching, photo based rendering (through a principled implementation), and more.

I've tried my hand a few toy path-tracers and ray-marchers in the past, and I wanted to write something more than just a toy.
Having stumbled upon *Ray Tracing in One Weekend* and Rust at around the same time, I decided to try my hand at Rust by implementing my own rendering engine from scratch.

Although Keikan uses elements from *Ray Tracing in One Weekend*, in of itself, it's fairly unique, as I wrote a large portion of the code based on past experience with rendering engines.

## Design Goals
Keikan is able to render a large variety of objects, including fractals to near-infinite precision, due to its support of both path-tracing and ray-marching.

Ray-marching is rendering through the use [signed distance fields](https://iquilezles.org/www/articles/distfunctions/distfunctions.htm). I personally got interested in computer graphics through writing ray-marching renderer, so I decided that if I wrote another rendering engine, I'd have include ray-marching as a technique.

## Results

> TODO
