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
| **[rulinalg](https://crates.io/crates/rulinalg)** | 0.068 | *0.117* | *0.582* |
| **[ndarray](https://crates.io/crates/ndarray)** | 0.073 | 0.121 | 0.592 |
| **[nalgebra](https://crates.io/crates/nalgebra)** | 0.009 | 0.193 | 1.131 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    vek = "0.9.5"      # Generic 2D-3D math swiss army knife for game engines, with SIMD support and focus on convenience.
    cgmath = "0.16.1"              # A linear algebra and mathematics library for computer graphics.
    rulinalg = "0.4.2"         # A linear algebra library.
    ndarray = "0.12.1"            # An n-dimensional array for general elements and for numerics. Lightweight array views and slicing…
    nalgebra = "0.16.13"           # Linear algebra library with transformations and statically-sized or dynamically-sized matrices.

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`