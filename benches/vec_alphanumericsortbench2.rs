//!Sorting Vecs of with alphanumeric strings.
//!
//!* *Baseline*: 1 item.
//!* *Sorted*: 1,000 items, fully sorted already.
//!* *Random*: 1,000 items, randomly shuffled. (The shuffling is not part of the time).
//!* *Reverse*: 1,000 items, sorted in reverse order.

#[macro_use]
extern crate bencher;

mod _alphanumeric_sort;
mod _humanesort;

benchmark_group!(baseline, _alphanumeric_sort::baseline,
                           _humanesort::baseline);
benchmark_group!(sorted,   _alphanumeric_sort::sorted,
                           _humanesort::sorted);
benchmark_group!(random,   _alphanumeric_sort::random,
                           _humanesort::random);
benchmark_group!(reverse,  _alphanumeric_sort::reverse,
                           _humanesort::reverse);

benchmark_main!(baseline, sorted, random, reverse);
