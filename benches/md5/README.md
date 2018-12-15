# md5
| | baseline | lorem |
| --- | --- | --- |
| **[md_5](https://crates.io/crates/md_5)** | 0.21 | 82.172 |
| **[md5](https://crates.io/crates/md5)** | 0.927 | 122.775 |

Speed units are in microseconds per iteration.

* Baseline: Hash "Hello".
* Lorem: Hash 60k of Lorem Ipsum.

Crate versions tested:

    md-5 = "0.8.0"     # MD5 hash function
    md5 = "0.6.1"                 # The package provides the MD5 hash function.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`