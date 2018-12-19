//!Sorting Vecs of u32.
//!
//!* *Baseline*: 1 item.
//!* *Sorted*: 10,000 items, fully sorted already.
//!* *Random*: 10,000 items, randomly shuffled. (The shuffling is not part of the time).
//!* *Reverse*: 10,000 items, sorted in reverse order.

#[macro_use]
extern crate bencher;

mod _std;
mod _afsort;
mod _cycle_sort;
mod _dmsort;
mod _ironsort;
mod _quick_sort;
mod _rdxsort;
mod _rust_quicksort;
mod _sortrs;

benchmark_group!(baseline, _std::baseline,
                           _afsort::baseline,
                           _cycle_sort::baseline,
                           _dmsort::baseline,
                           _ironsort::baseline,
                           _quick_sort::baseline,
                           _rdxsort::baseline,
                           _rust_quicksort::baseline,
                           _sortrs::baseline);
benchmark_group!(sorted,   _std::sorted,
                           _afsort::sorted,
                           _cycle_sort::sorted,
                           _dmsort::sorted,
                           _ironsort::sorted,
                           _quick_sort::sorted,
                           _rdxsort::sorted,
                           _rust_quicksort::sorted,
                           _sortrs::sorted);
benchmark_group!(random,   _std::random,
                           _afsort::random,
                           _cycle_sort::random,
                           _dmsort::random,
                           _ironsort::random,
                           _quick_sort::random,
                           _rdxsort::random,
                           _rust_quicksort::random,
                           _sortrs::random);
benchmark_group!(reverse,  _std::reverse,
                           _afsort::reverse,
                           _cycle_sort::reverse,
                           _dmsort::reverse,
                           _ironsort::reverse,
                           _quick_sort::reverse,
                           _rdxsort::reverse,
                           _rust_quicksort::reverse,
                           _sortrs::reverse);

benchmark_main!(baseline, sorted, random, reverse);
