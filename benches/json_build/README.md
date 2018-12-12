# json_build
| | baseline | serial | nested |
| --- | --- | --- | --- |
| **[json_in_type](https://crates.io/crates/json_in_type)** | 0.08 | 1.65 | 1.408 |
| **[json](https://crates.io/crates/json)** | 0.125 | 6.453 | 8.27 |
| **[serde_json](https://crates.io/crates/serde_json)** | 0.623 | - | 31.136 |

Speed units are in microseconds per iteration.

* Baseline: Simple json with just one entry.
* Serial: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
* Nested: 100 nested entries, each inside of the other.

Crate versions tested:

    json_in_type = "1.0.0"           # a library for fast json serialization
    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`