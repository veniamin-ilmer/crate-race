//!Doing a dot product operation on different size vectors. All floats.
//!
//!* *Baseline*: 3 dimension vector * 3 dimension vector
//!* *Hundred*: 100 dimension vector * 100 dimension vector
//!* *Thousand*: 1000 dimension vector * 1000 dimension vector
//!
//!Note - `cgmath` and `vek` doesn't allow to do more than 4 dimensions.

#[macro_use]
extern crate bencher;

#[macro_use]
extern crate rulinalg;

mod _nalgebra;
mod _rulinalg;
mod _ndarray;
mod _cgmath;
mod _vek;

benchmark_group!(baseline, _nalgebra::baseline, _rulinalg::baseline, _ndarray::baseline, _cgmath::baseline, _vek::baseline);
benchmark_group!(hundred, _nalgebra::hundred, _rulinalg::hundred, _ndarray::hundred);
benchmark_group!(thousand, _nalgebra::thousand, _rulinalg::thousand, _ndarray::thousand);

benchmark_main!(baseline, hundred, thousand);
