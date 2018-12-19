# vec_intsort
Sorting Vecs of u32.

* *Baseline*: 1 item.
* *Sorted*: 10,000 items, fully sorted already.
* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
* *Reverse*: 10,000 items, sorted in reverse order.

| | baseline | sorted | random | reverse |
| --- | --- | --- | --- | --- |
| **std** | 0.036 | *6.225* | 215.622 | *8.92* |
| **[dmsort](https://crates.io/crates/dmsort)** | 0.036 | 49.684 | 239.175 | 22.758 |
| **[rdxsort](https://crates.io/crates/rdxsort)** | 1.481 | 193.324 | *163.289* | 193.077 |
| **[sortrs](https://crates.io/crates/sortrs)** | 0.037 | 76.794 | 433.34 | 71.508 |
| **[afsort](https://crates.io/crates/afsort)** | 0.039 | 185.665 | 268.71 | 211.819 |
| **[ironsort](https://crates.io/crates/ironsort)** | *0.035* | 72.968 | 527.78 | 85.518 |
| **[rust_quicksort](https://crates.io/crates/rust_quicksort)** | 0.038 | 93.335 | 548.788 | 2888.365 |
| **[quick_sort](https://crates.io/crates/quick_sort)** | 0.035 | 106.422 | 493.34 | 23203.443 |
| **[cycle_sort](https://crates.io/crates/cycle_sort)** | 0.036 | 14161.158 | 42360.43 | 24763.832 |

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

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`