# bigint_multiplication
Multiplying numbers larger than what's possible with a normal 64 bit processor.

* Baseline: Multiply 1 * 1
* Fact50: Take the factorial of 50. (Gets up to 256 bits)
* Fact95: Take the factorial of 95. (Gets up to 512 bits)

| | baseline | fact50 | fact95 |
| --- | --- | --- | --- |
| **[num_bigint](https://crates.io/crates/num_bigint)** | 0.262 | *2.992* | *6.04* |
| **[numext_fixed_uint](https://crates.io/crates/numext_fixed_uint)** | *0.147* | 18.87 | 94.143 |
| **[uint](https://crates.io/crates/uint)** | 0.17 | 115.646 | 935.56 |

Speed units are in microseconds per iteration. Less is better.

##Crate versions

    num-bigint = "0.2.2"        # Big integer implementation for Rust
    numext-fixed-uint = "0.1.0"     # Fixed-size uint types.
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`