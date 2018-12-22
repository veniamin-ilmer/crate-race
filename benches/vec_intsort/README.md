# vec_intsort
Sorting Vecs of u32.

* *Baseline*: 1 item.
* *Sorted*: 10,000 items, fully sorted already.
* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
* *Reverse*: 10,000 items, sorted in reverse order.

| | baseline | sorted | reverse | random |
| --- | --- | --- | --- | --- |
| **std** | 0.037 | *6.265* | *8.871* | 215.558 |
| **[dmsort](https://crates.io/crates/dmsort)** | 0.036 | 50.285 | 22.778 | 239.727 |
| **[rdxsort](https://crates.io/crates/rdxsort)** | 1.466 | 201.081 | 201.035 | *164.252* |
| **[sortrs](https://crates.io/crates/sortrs)** | 0.036 | 76.295 | 74.888 | 430.266 |
| **[afsort](https://crates.io/crates/afsort)** | 0.039 | 182.98 | 210.82 | 269.263 |
| **[ironsort](https://crates.io/crates/ironsort)** | 0.036 | 80.296 | 94.191 | 515.411 |
| **[rust_quicksort](https://crates.io/crates/rust_quicksort)** | 0.037 | 94.138 | 2802.348 | 545.802 |
| **[quick_sort](https://crates.io/crates/quick_sort)** | *0.035* | 111.094 | 22827.508 | 503.4 |
| **[cycle_sort](https://crates.io/crates/cycle_sort)** | 0.036 | 14120.508 | 33609.664 | 66057.58 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    dmsort = "1.0.0"    # Fast adaptive sorting for when most of your data is already in order. dmsort can be 2-5 times faster than R…
    rdxsort = "0.3.0"    # Fast Radix Sort
    sortrs = "0.0.5"    # An introspective sort implementation. 
    afsort = "0.3.0"    # American Flag sort implementation for faster sorting of Strings. 
    ironsort = "0.1.0"    # Implementation of a fast in-place sorting algorithm
    rust_quicksort = "0.1.0"    # Quicksort library for Rust language.
    quick_sort = "0.2.1"          # In place quick sort
    cycle-sort = "0.2.0"           # Simple generic Cycle sort implementation

Compiled on: `rustc 1.31.1 (b6c32da9b 2018-12-18)`