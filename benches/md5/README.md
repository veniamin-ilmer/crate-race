# md5
| | baseline | lorem |
| --- | --- | --- |
| **md_5** | 0.213 | 85.237 |
| **md5** | 0.94 | 123.929 |

Speed units are in microseconds per iteration.

* Baseline: Hash "Hello".
* Lorem: Hash 60k of Lorem Ipsum.

Crate versions tested:

    md-5 = "0.8.0"     # MD5 hash function
    md5 = "0.6.0"                 # The package provides the MD5 hash function.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)
`