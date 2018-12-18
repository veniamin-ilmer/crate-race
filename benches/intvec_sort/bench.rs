//!Sorting Vecs of i64.
//!
//!* *Baseline_small*: 10 items. Fully sorted already.
//!* *Random_small = 10 items. Completely random.
//!* *Reverse_small = 10 items. Sorted in reverse order.
//!* *Baseline_big = 10,000 items. Fully sorted already.
//!* *Random_big = 10,000 items. Completely random.
//!* *Reverse_big = 10,000 items. Sorted in reverse order.

#[macro_use]
extern crate bencher;

benchmark_group!(baseline, _modhere::baseline);

benchmark_main!(baseline);
