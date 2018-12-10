| | json | serde_json | json5 |
| --- | --- | --- | --- |
| **baseline** | 0.196 | 0.291 | 2.352 |
| **serial** | 7.937 | 23.002 | 158.744 |
| **nested** | 13.293 | 26.17 | 153.521 |

Speed units are in microseconds per iteration

Compiled on:

cargo 1.31.0 (339d9f9c8 2018-11-16)

Crate versions tested:

    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format
    json5 = "0.2.2"            # A Rust JSON5 serializer and deserializer which speaks Serde.
