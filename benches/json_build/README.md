# json_build
Building JSON strings using objects.

* **Baseline**: Simple json with just one entry.
* **Serial**: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
* **Nested**: 100 nested entries, each inside of the other.

| | baseline | serial | nested |
| --- | --- | --- | --- |
| **[json_in_type](https://crates.io/crates/json_in_type)** | *0.083* | *1.775* | *1.368* |
| **[json](https://crates.io/crates/json)** | 0.128 | 6.478 | 8.269 |
| **[serde_json](https://crates.io/crates/serde_json)** | 0.724 | - | 31.598 |

Speed units are in microseconds per iteration. Less is better.

## Related Functions

* [json_parse](../json_parse)

## Crate versions

    json_in_type = "1.0.0"           # a library for fast json serialization
    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`