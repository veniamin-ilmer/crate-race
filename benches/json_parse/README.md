# json_parse
Building JSON strings using objects.

* Baseline: Simple json with just one entry.
* Serial: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
* Nested: 100 nested entries, each inside of the other.

| | baseline | serial | nested |
| --- | --- | --- | --- |
| **[json](https://crates.io/crates/json)** | *0.183* | *7.342* | *11.791* |
| **[serde_json](https://crates.io/crates/serde_json)** | 0.286 | 22.549 | 25.625 |
| **[json5](https://crates.io/crates/json5)** | 2.319 | 142.145 | 150.489 |

Speed units are in microseconds per iteration. Less is better.

##Crate versions

    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format
    json5 = "0.2.2"            # A Rust JSON5 serializer and deserializer which speaks Serde.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`