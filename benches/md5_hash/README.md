# md5_hash
MD5 hashing

* **Baseline**: Hash "Hello".
* **Lorem**: Hash 60k of Lorem Ipsum.

| | baseline | lorem |
| --- | --- | --- |
| **[md_5](https://crates.io/crates/md_5)** | *0.204* | *83.54* |
| **[md5](https://crates.io/crates/md5)** | 1.039 | 127.927 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    md-5 = "0.8.0"     # MD5 hash function
    md5 = "0.6.1"                 # The package provides the MD5 hash function.

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`