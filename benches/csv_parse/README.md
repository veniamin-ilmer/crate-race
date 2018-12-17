# csv_parse
Parsing CSVs, and retrieving values at specific rows/columns.

* **Baseline**: Simple csv with just one entry.
* **Rows**: 1 header, 100 rows
* **Headers**: 100 headers, 1 row

| | baseline | headers | rows |
| --- | --- | --- | --- |
| **[quick_csv](https://crates.io/crates/quick_csv)** | *0.405* | - | *90.611* |
| **[csv](https://crates.io/crates/csv)** | 15.363 | *37.9* | 157.61 |
| **[simple_csv](https://crates.io/crates/simple_csv)** | 0.415 | 137.242 | 170.633 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    quick-csv = "0.1.6"               # quick csv reader and decoder
    csv = "1.0.5"              # Fast CSV parsing with support for serde.
    simple_csv = "0.0.15"             # A simple CSV parsing implementation

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`