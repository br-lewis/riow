
This is an implementation of Peter Shirley's [Ray Tracing in One
Weekend](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf)
done in Rust.

It's mostly a direct translation from the C++ shown in the PDF with some light
modifications to make it more Rusty like using `Option<T>` instead of
functions that take an out parameter and return `bool`. And from chapter 8
onward, pixel calculation is parallelized with Rayon to speed things up.

