# string_findbyte
Search for just 1 byte inside of a byte array.

* **Baseline**: Just one character string, length 1.
* **Big_Str_First**: 50kb string. Find character at the beginning of the string.
* **Big_Str_Last**: 50kb string. Find character at the end of the string.

| | big_str_first | baseline | big_str_last |
| --- | --- | --- | --- |
| **[memchr](https://crates.io/crates/memchr)** | *0.003* | *0.003* | *0.518* |
| **[jetscii](https://crates.io/crates/jetscii)** | 0.016 | 0.018 | 12.97 |

Speed units are in microseconds per iteration. Less is better.

## Related Functions

* [string_findsubstring](../string_findsubstring)

## Crate versions

    memchr = "2.1.2"           # Safe interface to memchr.
    jetscii = "0.4.3"    # A tiny library to efficiently search strings and byte slices for sets of ASCII characters or bytes.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`