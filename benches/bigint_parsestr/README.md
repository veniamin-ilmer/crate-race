# bigint_parsestr
Parsing large numbers from strings.

* Baseline: Convert "1" string into a number, and then back into a string again.
* Big_num: Convert a 149 digit string into a number, and then back into a string again.

| | baseline | big_num |
| --- | --- | --- |
| **[num_bigint](https://crates.io/crates/num_bigint)** | 0.256 | *1.261* |
| **[numext_fixed_uint](https://crates.io/crates/numext_fixed_uint)** | 0.156 | 91.223 |
| **[uint](https://crates.io/crates/uint)** | *0.153* | 910.022 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    num-bigint = "0.2.2"        # Big integer implementation for Rust
    numext-fixed-uint = "0.1.0"     # Fixed-size uint types.
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`