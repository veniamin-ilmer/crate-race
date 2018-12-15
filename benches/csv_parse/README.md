# csv_parse
| | baseline | headers | rows |
| --- | --- | --- | --- |
| **[quick_csv](https://crates.io/crates/quick_csv)** | 0.398 | 12.817 | 9.49 |
| **[simple_csv](https://crates.io/crates/simple_csv)** | 0.417 | 14.476 | 16.374 |
| **[csv](https://crates.io/crates/csv)** | 15.446 | 18.22 | 29.694 |

Speed units are in microseconds per iteration.

* Baseline: Simple csv with just one entry.
* Rows: 1 header, 100 rows
* Headers: 100 headers, 1 row

Crate versions tested:

    quick-csv = "0.1.6"               # quick csv reader and decoder
    simple_csv = "0.0.15"             # A simple CSV parsing implementation
    csv = "1.0.3"              # Fast CSV parsing with support for serde.

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`