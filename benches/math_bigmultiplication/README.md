# math_bigmultiplication
| | baseline | fact50 | fact95 |
| --- | --- | --- | --- |
| **[num_bigint](https://crates.io/crates/num_bigint)** | 0.261 | 2.954 | 6.008 |
| **[numext_fixed_uint](https://crates.io/crates/numext_fixed_uint)** | 0.154 | 19.076 | 94.673 |
| **[uint](https://crates.io/crates/uint)** | 0.179 | 114.373 | 963.61 |

Speed units are in microseconds per iteration.

* Baseline: Multiply 1 * 1
* Fact50: Take the factorial of 50.
* Fact95: Take the factorial of 95.

Crate versions tested:

    num-bigint = "0.2.2"        # Big integer implementation for Rust
    numext-fixed-uint = "0.1.0"     # Fixed-size uint types.
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`