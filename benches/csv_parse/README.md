# csv_parse
Parsing CSVs, and retrieving values at specific rows/columns.

* **Baseline**: Simple csv with just one entry.
* **Rows**: 1 header, 100 rows
* **Headers**: 100 headers, 1 row

| | baseline | headers | rows |
| --- | --- | --- | --- |
| **[quick_csv](https://crates.io/crates/quick_csv)** | *0.403* | - | *92.382* |
| **[csv](https://crates.io/crates/csv)** | 17.078 | *39.463* | 160.045 |
| **[simple_csv](https://crates.io/crates/simple_csv)** | 0.412 | 130.985 | 170.141 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    quick-csv = "0.1.6"               # quick csv reader and decoder
    csv = "1.0.5"              # Fast CSV parsing with support for serde.
    simple_csv = "0.0.15"             # A simple CSV parsing implementation

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`