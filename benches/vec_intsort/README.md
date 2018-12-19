# vec_intsort
Sorting Vecs of u32.

* *Baseline*: 1 item.
* *Sorted*: 10,000 items, fully sorted already.
* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
* *Reverse*: 10,000 items, sorted in reverse order.

| | baseline | sorted | random | reverse |
| --- | --- | --- | --- | --- |
| **[std](https://crates.io/crates/std)** | 0.036 | *6.231* | 216.179 | *8.918* |
| **[dmsort](https://crates.io/crates/dmsort)** | 0.036 | 50.817 | 240.989 | 23.166 |
| **[rdxsort](https://crates.io/crates/rdxsort)** | 1.506 | 199.138 | *163.689* | 199.128 |
| **[sortrs](https://crates.io/crates/sortrs)** | 0.036 | 77.44 | 436.393 | 70.922 |
| **[afsort](https://crates.io/crates/afsort)** | 0.039 | 181.692 | 272.963 | 212.742 |
| **[ironsort](https://crates.io/crates/ironsort)** | *0.035* | 72.616 | 524.534 | 87.167 |
| **[rust_quicksort](https://crates.io/crates/rust_quicksort)** | 0.037 | 93.81 | 548.355 | 2805.51 |
| **[quick_sort](https://crates.io/crates/quick_sort)** | 0.035 | 105.564 | 496.756 | 23290.195 |
| **[cycle_sort](https://crates.io/crates/cycle_sort)** | 0.036 | 14171.731 | 42493.77 | 24876.83 |

Speed units are in microseconds per iteration. Less is better.

## Crate versions

    optional = "0.5.0"                 # This crate supplies a number of Option-like primitive types
    dmsort = "1.0.0"    # Fast adaptive sorting for when most of your data is already in order. dmsort can be 2-5 times faster than R…
    rdxsort = "0.3.0"    # Fast Radix Sort
    sortrs = "0.0.5"    # An introspective sort implementation. 
    afsort = "0.3.0"    # American Flag sort implementation for faster sorting of Strings. 
    ironsort = "0.1.0"    # Implementation of a fast in-place sorting algorithm
    rust_quicksort = "0.1.0"    # Quicksort library for Rust language.
    quick_sort = "0.2.1"          # In place quick sort
    cycle-sort = "0.2.0"           # Simple generic Cycle sort implementation

Compiled on: `cargo 1.31.0 (339d9f9c8 2018-11-16)`