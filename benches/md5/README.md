# md5
MD5 hashing

* **Baseline**: Hash "Hello".
* **Lorem**: Hash 60k of Lorem Ipsum.

| | baseline | lorem |
| --- | --- | --- |
| **[md_5](https://crates.io/crates/md_5)** | *0.209* | *83.275* |
| **[md5](https://crates.io/crates/md5)** | 0.949 | 125.097 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    md-5 = "0.8.0"     # MD5 hash function
    md5 = "0.6.1"                 # The package provides the MD5 hash function.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`