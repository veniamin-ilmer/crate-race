# vec_intsort
Sorting Vecs of u32.

* *Baseline*: 1 item.
* *Sorted*: 10,000 items, fully sorted already.
* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
* *Reverse*: 10,000 items, sorted in reverse order.

| | baseline | sorted | random | reverse |
| --- | --- | --- | --- | --- |
| **std** | 0.037 | *6.098* | 215.711 | *9.245* |
| **[dmsort](https://crates.io/crates/dmsort)** | 0.037 | 50.397 | 239.457 | 23.197 |
| **[rdxsort](https://crates.io/crates/rdxsort)** | 1.533 | 204.462 | *169.008* | 199.266 |
| **[sortrs](https://crates.io/crates/sortrs)** | *0.036* | 78.146 | 429.932 | 72.866 |
| **[afsort](https://crates.io/crates/afsort)** | 0.04 | 186.268 | 272.734 | 212.396 |
| **[ironsort](https://crates.io/crates/ironsort)** | 0.036 | 73.408 | 521.591 | 87.882 |
| **[rust_quicksort](https://crates.io/crates/rust_quicksort)** | 0.037 | 92.64 | 548.581 | 2810.284 |
| **[quick_sort](https://crates.io/crates/quick_sort)** | 0.036 | 111.435 | 488.882 | 23429.313 |
| **[cycle_sort](https://crates.io/crates/cycle_sort)** | 0.036 | 14124.811 | 42719.34 | 24907.951 |

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