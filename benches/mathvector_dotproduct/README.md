# mathvector_dotproduct
Doing a dot product operation on different size vectors. All floats.

* *Baseline*: 3 dimension vector * 3 dimension vector
* *Hundred*: 100 dimension vector * 100 dimension vector
* *Thousand*: 1000 dimension vector * 1000 dimension vector

Note - `cgmath` and `vek` doesn't allow to do more than 4 dimensions.

| | baseline | hundred | thousand |
| --- | --- | --- | --- |
| **[vek](https://crates.io/crates/vek)** | *0* | - | - |
| **[cgmath](https://crates.io/crates/cgmath)** | 0 | - | - |
| **[ndarray](https://crates.io/crates/ndarray)** | 0.067 | *0.115* | *0.627* |
| **[rulinalg](https://crates.io/crates/rulinalg)** | 0.067 | 0.117 | 0.642 |
| **[nalgebra](https://crates.io/crates/nalgebra)** | 0.009 | 0.195 | 1.165 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    vek = "0.9.5"    # Generic 2D-3D math swiss army knife for game engines, with SIMD support and focus on convenience.
    cgmath = "0.16.1"              # A linear algebra and mathematics library for computer graphics.
    ndarray = "0.12.1"            # An n-dimensional array for general elements and for numerics. Lightweight array views and slicing…
    rulinalg = "0.4.2"         # A linear algebra library.
    nalgebra = "0.16.12"           # Linear algebra library with transformations and statically-sized or dynamically-sized matrices.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`