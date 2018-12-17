# Crate Race
Wondering which crate can do some functionality faster?

Crate race feeds the same kind of inputs into crate functions, and benchmarks the results.

[Check out a list of functions here](benches).

Each benchmark comes with the actual code used to run each of the functions.

You can look through the code to judge which crate to use.

## Forewarning

Just because one crate has a faster speed for a specific function, does not automatically make that crate "better".

Other crates may specialize in other forms of functionality. Other crates may also be more user friendly to use.

Try not to generalize after just looking at one benchmark.

## Benchmarking Machine Specs

* Operating System: Windows 7 Professional
* Processor: [Intel Core i5-6600 Processor](https://en.wikichip.org/wiki/intel/core_i5/i5-6600) quad-core 64-bit x86, 3.3 GHz base frequency.
* Motherboard: [Gigabyte LGA1151 Intel B150 Micro ATX](https://www.amazon.com/gp/product/B017WL3V4Q/)
* RAM: [GeIL EVO POTENZA 16GB DDR4 SDRAM](https://www.amazon.com/gp/product/B01EWBFZSE) DDR4 2400 (PC4 19200)
* Graphics Card: [EVGA GeForce GTX 960](https://www.amazon.com/gp/product/B01BX3FAWU/) 4GB ACX 2.0+
* Hard Drive: [Seagate 3TB Desktop HDD SATA 6Gb/s 64MB Cache](https://www.amazon.com/gp/product/B005T3GRLY/)

## Rust and Crate Updates
All of the benchmarks here are automatically generated.

Every day Crate Race checks for new version of Rust or Crates.

If an update is detected, Crate Race will rerun the benchmarks test and update github.

## Limitations
### Only Shared functions
Crate Race will not benchmark every single crate's unique functions.

It will only benchmark functions that match other crates' functions.

If a crate has an extra function that other crates do not have, Crate Race will not benchmark it.

### Correctness checking
Although Crate Race checks that the function returned a correct value expected for that function, it will not test the functions' edge cases being handled correctly.

We are benchmarking, not QAing.
