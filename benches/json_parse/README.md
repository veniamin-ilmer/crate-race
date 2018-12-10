# json_parse
| | baseline | serial | nested |
| --- | --- | --- | --- |
| **json** | 0.187 | 7.496 | 11.968 |
| **serde_json** | 0.292 | 23.159 | 26.346 |
| **json5** | 2.287 | 143.534 | 171.783 |

Speed units are in microseconds per iteration

Crate versions tested:

    json = "0.11.13"                          # JSON implementation in Rust
    serde_json = "1.0.33"                      # A JSON serialization file format
    json5 = "0.2.2"            # A Rust JSON5 serializer and deserializer which speaks Serde.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)
`