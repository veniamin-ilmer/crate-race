# json_parse
| | baseline | serial | nested |
| --- | --- | --- | --- |
| **json** | 0.184 | 7.367 | 11.903 |
| **serde_json** | 0.295 | 23.021 | 26.343 |
| **json5** | 2.336 | 146.681 | 153.157 |

Speed units are in microseconds per iteration.

* Baseline: Simple json with just one entry.
* Serial: Opposite of nested. 100 entries listed in a "flat" format, one by one, without any tree structure.
* Nested: 100 nested entries, each inside of the other.

Crate versions tested:

    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format
    json5 = "0.2.2"            # A Rust JSON5 serializer and deserializer which speaks Serde.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)
`