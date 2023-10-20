# Rust Ray tracer

Ray tracer implemented in Rust from https://github.com/RayTracing/raytracing.github.io
The code is quite simple, a lot of rust improvement could be done and probably make everything faster.
The two first lesson were done so far, the ray tracer in one weekend and the Ray tracer in one week.

# Differences

So far the biggest difference is the handler of images. Since the library 'IMAGE' contains all the necessary functions, so I just used directly their class.

# To improve

Pretty much everything. The code follow almost line by line the courses, so everything feels very "C++". I am sure there are better ways of doing things.

Creating a picture with good quality takes forever. It could be interesting to be able to send the computation to a GPU once the code is cleaned up.
