# math_bigint
| | baseline | fact95 |
| --- | --- | --- |
| **[num_bigint](https://crates.io/crates/num_bigint)** | 0.215 | 6.023 |
| **[numext_fixed_uint](https://crates.io/crates/numext_fixed_uint)** | 0.14 | 93.463 |
| **[uint](https://crates.io/crates/uint)** | 0.144 | 922.389 |

Speed units are in microseconds per iteration.

* Baseline: Add 1 + 1
* Fact95: Take the factorial of 95.

Crate versions tested:

    num-bigint = "0.2.2"        # Big integer implementation for Rust
    numext-fixed-uint = "0.1.0"     # Fixed-size uint types.
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`