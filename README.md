# Crate Race
Are you looking at two Rust Crates that do the same thing?

Want to know which crate to use?

Well Crate Race might help!

Crate race feeds the same kind of inputs into crate functions, and benchmarks the results.

[Check out a list of functions here](benches).

Each benchmark comes with the actual code used to run each of the functions.

You can look through the code to judge which crate to use.

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

It will only benchmark functions that match other relatively popular crates' functions.

If a crate has an extra function that other crates do not have, Crate Race will not benchmark it.

### Correctness checking
Although Crate Race checks that the function returned a correct value expected for that function, it will not test the functions' edge cases being handled correctly.

We are benchmarking, not QAing.

### Subjective Comparison
Some crates may be more easy to use than others, or have some other additional functionality that other crates don't have.

Crate Race will not be rating these factors. It will only be benchmarking the functions.

However the code used to benchmark the function will always be provided, so you can look at the code and judge for yourself how hard or easy it will be to use.

## Contributing
Crate Race will only benchmark crates that share functionality with other crates.

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

If there are multiple tests for the function, feel free to add in these tests into this file.

Check out [how to add a crate](add_crate.md) for more details of all the other files that need to be modified.
