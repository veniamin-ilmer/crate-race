# big_arithmetic
| | baseline | fact95 |
| --- | --- | --- |
| **num_bigint** | 0.229 | 6.6 |
| **uint** | 0.147 | 941.659 |

Speed units are in microseconds per iteration.

* Baseline: Add 1 + 1
* Fact95: Take the factorial of 95.

Crate versions tested:

    num-bigint = "0.2.1"        # Big integer implementation for Rust
    uint = "0.5.0"                     # Large fixed-size integers arithmetics

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)
`