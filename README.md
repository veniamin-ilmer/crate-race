# Crate Race
Are you looking at two Rust Crates that do the same thing?

Want to know which crate to use?

Well Crate Race might help!

Crate race feeds the same kind of inputs into crate functions, and benchmarks the results.

[Check out a list of functions here](benches).

## Benchmarking Machine Specs

* Operating System: Windows 7 Professional (Support for multiple operating systems planned for the future.)
* Processor: [Intel Core i5-6600 Processor](https://en.wikichip.org/wiki/intel/core_i5/i5-6600) quad-core 64-bit x86, 3.3 GHz base frequency.
* Motherboard: [Gigabyte LGA1151 Intel B150 Micro ATX](https://www.amazon.com/gp/product/B017WL3V4Q/)
* RAM: (GeIL EVO POTENZA 16GB DDR4 SDRAM)[https://www.amazon.com/gp/product/B01EWBFZSE] DDR4 2400 (PC4 19200)
* Graphics Card: [EVGA GeForce GTX 960](https://www.amazon.com/gp/product/B01BX3FAWU/) 4GB ACX 2.0+
* Hard Drive: [Seagate 3TB Desktop HDD SATA 6Gb/s 64MB Cache](https://www.amazon.com/gp/product/B005T3GRLY/)

## Rust and Crate Updates
All of the benchmarks here are automatically generated.

Every day Crate Race checks for new version of Rust or Crates.

If an update is detected, Crate Race will rerun the benchmarks test and update github.

## Contributing
Crate Race will only benchmark crates that share functionality with other crates.

It will only test functions shared between the crates.

If a crate has an extra function that other crates do not have, Crate Race will not benchmark it.

If you have such shared crate functions, please feel free to add in a Pull Request with the extra crate.

Please adhere to the standard visible with all other benchmarks in this project:

Only one of the crates functions should be tested per bench folder.

Name it "_[cratename].rs" inside the folder.

Include this template code in there:

    use bencher::Bencher;

    pub fn baseline(b: &mut Bencher) {
      b.iter(|| {
        //Run baseline crate function here
        //Add assert_eq!() code here to test the output of the function
      });
    }

If there are multiple tests for the function, feel free to add in these tests into this folder.
