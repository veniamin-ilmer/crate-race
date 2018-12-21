# bigint_arithmetic
Multiplying numbers larger than what's possible with a normal 64 bit processor.

* **Baseline**: Add 1 + 1
* **Fact50**: Take the factorial of 50. (Gets up to 256 bits)
* **Fact95**: Take the factorial of 95. (Gets up to 512 bits)

Note: We cannot include GMP or unstable crates. GMP can't be included because it is not crossplatform (not available in Windows).

| | baseline | fact50 | fact95 |
| --- | --- | --- | --- |
| **[num_bigint](https://crates.io/crates/num_bigint)** | 0.13 | *10.131* | *28.544* |
| **[numext_fixed_uint](https://crates.io/crates/numext_fixed_uint)** | *0.009* | 74.353 | 426.554 |
| **[uint](https://crates.io/crates/uint)** | 0.016 | 125.211 | 1031.125 |

Speed units are in microseconds per iteration. Less is better.

## Related Functions

* [bigint_parsestr](../bigint_parsestr)

## Crate versions

    num-bigint = "0.2.2"        # Big integer implementation for Rust
    numext-fixed-uint = "0.1.2"     # Fixed-size uint types.
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`